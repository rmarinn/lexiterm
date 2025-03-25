//! Handles background search queries and sends results back to the main thread.
//!
//! This worker listens for search queries via a [`Receiver<String>`], processes them
//! using a [`WordTrie`], and sends the results back via a [`Sender<Vec<String>>`].
//!
//! The worker implements **debouncing**, ensuring that rapid consecutive queries
//! are ignored except for the most recent one within a short time window.

use anyhow::{anyhow, Result};
use crossbeam::channel::{Receiver, Sender};
use std::time::Duration;
use word_trie::ScoredWordTrie;

/// The debounce duration for processing search queries.
///
/// If a new query arrives within this duration, the previous query is discarded.
static DEBOUNCE_DUR: Duration = Duration::from_millis(100);

#[derive(Debug)]
pub struct QueryRequest {
    pub letters: String,
    pub regex: String,
}

#[derive(Debug)]
pub struct QueryResponse {
    pub words: Vec<String>,
}

/// Listens for incoming search queries and processes only the most recent one.
///
/// This function continuously receives search queries from `query_rx`, applies
/// **debouncing** to ignore outdated queries, processes the latest one using a
/// [`WordTrie`], and then sends the sorted results back through `result_tx`.
pub fn search_worker(
    word_trie: ScoredWordTrie,
    query_rx: Receiver<QueryRequest>,
    result_tx: Sender<Result<QueryResponse>>,
) {
    loop {
        // Block until at least one query arrives
        let Ok(mut query) = query_rx.recv() else {
            return;
        };

        // Keep receiving queries within the debounce window
        while let Ok(new_query) = query_rx.recv_timeout(DEBOUNCE_DUR) {
            query = new_query
        }

        // Process only the most recent query
        let words = if query.regex.is_empty() {
            Ok(word_trie
                .get_words(&query.letters)
                .into_iter()
                .map(|(word, score)| format!("{}:{}", word, score))
                .collect())
        } else {
            word_trie
                .get_word_matches(&query.letters, &query.regex)
                .map(|words| {
                    words
                        .into_iter()
                        .map(|(word, score)| format!("{}:{}", word, score))
                        .collect()
                })
                .map_err(|e| anyhow!("invalid regex: {e}"))
        };

        let resp = words.map(|words| QueryResponse { words });

        if result_tx.send(resp).is_err() {
            break;
        }
    }
}
