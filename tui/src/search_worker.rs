//! Handles background search queries and sends results back to the main thread.
//!
//! This worker listens for search queries via a [`Receiver<String>`], processes them
//! using a [`WordTrie`], and sends the results back via a [`Sender<Vec<String>>`].
//!
//! The worker implements **debouncing**, ensuring that rapid consecutive queries
//! are ignored except for the most recent one within a short time window.

use crossbeam::channel::{Receiver, Sender};
use std::time::Duration;
use word_trie::ScoredWordTrie;

/// The debounce duration for processing search queries.
///
/// If a new query arrives within this duration, the previous query is discarded.
static DEBOUNCE_DUR: Duration = Duration::from_millis(100);

/// Listens for incoming search queries and processes only the most recent one.
///
/// This function continuously receives search queries from `query_rx`, applies
/// **debouncing** to ignore outdated queries, processes the latest one using a
/// [`WordTrie`], and then sends the sorted results back through `result_tx`.
///
/// # Arguments
///
/// * `query_rx` - A channel receiver for incoming search queries.
/// * `result_tx` - A channel sender to send the processed search results.
///
/// # Behavior
///
/// - **Blocking:** This function runs indefinitely, waiting for new queries.
/// - **Debouncing:** If multiple queries arrive within [`DEBOUNCE_DUR`], only the latest one is processed.
/// - **Sorting:** Results are sorted in descending order of word length.
///
/// # Termination
///
/// - If `query_rx` is closed, the function exits.
/// - If `result_tx` is closed, the function exits.
///
/// # Example
///
/// ```no_run
/// use crossbeam::channel;
/// use std::thread;
///
/// let (query_tx, query_rx) = channel::bounded::<String>(100);
/// let (result_tx, result_rx) = channel::bounded::<Vec<String>>(10);
///
/// thread::spawn(move || {
///     search_worker(query_rx, result_tx);
/// });
///
/// query_tx.send("hello".to_string()).unwrap();
/// ```
pub fn search_worker(
    word_trie: ScoredWordTrie,
    query_rx: Receiver<String>,
    result_tx: Sender<Vec<String>>,
) {
    // let word_trie = WordTrie::new_from_file(Path::new("../words.txt"));

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
        let words = word_trie
            .get_words_sorted(&query)
            .into_iter()
            .map(|(word, score)| format!("{}:{}", word, score))
            .collect();

        if result_tx.send(words).is_err() {
            break;
        }
    }
}
