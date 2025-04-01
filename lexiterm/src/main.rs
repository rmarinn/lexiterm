mod app_manager;
mod input_processing;
mod search_worker;
mod tui_renderer;

use anyhow::{anyhow, Result};
use app_manager::*;
use crossbeam::channel;
use input_processing::listen_and_process;
use lexicon::ScoredWordTrie;
use search_worker::{search_worker, QueryRequest, QueryResponse};
use std::path::Path;
use std::thread;
use tui_renderer::*;

fn main() -> Result<()> {
    let (query_tx, query_rx) = channel::bounded::<QueryRequest>(100);
    let (result_tx, result_rx) = channel::bounded::<QueryResponse>(30);

    let words_file_path = Path::new("./words.txt");
    let scores_file_path = Path::new("./char_scores.txt");
    let word_trie = ScoredWordTrie::new_from_files(words_file_path, scores_file_path)?;

    let search_handle = thread::spawn(move || {
        search_worker(word_trie, query_rx, result_tx);
    });

    let state_mngr = AppManager::default();
    let tui_renderer = TuiRenderer::default();

    let listener_result = listen_and_process(state_mngr, tui_renderer, &query_tx, &result_rx);

    // Ensure worker sees EOF and exits
    drop(query_tx);
    ratatui::restore();

    search_handle
        .join()
        .map_err(|e| anyhow!("Failed to join worker handle: {e:?}"))?;

    listener_result?;

    Ok(())
}
