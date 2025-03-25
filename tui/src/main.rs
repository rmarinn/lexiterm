mod input;
mod search_worker;
mod tui;

use anyhow::{anyhow, Result};
use crossbeam::channel;
use input::listen_and_process;
use search_worker::{search_worker, QueryRequest, QueryResponse};
use std::{path::Path, thread};
use tui::Tui;
use word_trie::ScoredWordTrie;

fn main() -> Result<()> {
    let (query_tx, query_rx) = channel::bounded::<QueryRequest>(0);
    let (result_tx, result_rx) = channel::bounded::<Result<QueryResponse>>(0);

    let words_file_path = Path::new("../words.txt");
    let scores_file_path = Path::new("../char_scores.txt");
    let word_trie = ScoredWordTrie::new_from_files(words_file_path, scores_file_path)?;

    let search_handle = thread::spawn(move || {
        search_worker(word_trie, query_rx, result_tx);
    });

    let tui = Tui::default();

    let listener_result = listen_and_process(&tui, &query_tx, &result_rx);

    // Ensure worker sees EOF and exits
    drop(query_tx);
    ratatui::restore();

    search_handle
        .join()
        .map_err(|e| anyhow!("Failed to join worker handle: {e:?}"))?;

    listener_result?;

    Ok(())
}
