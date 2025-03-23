mod input;
mod search_worker;
mod tui;

use anyhow::{anyhow, Result};
use crossbeam::channel;
use input::listen_and_process;
use search_worker::search_worker;
use std::thread;
use tui::Tui;

fn main() -> Result<()> {
    let (query_tx, query_rx) = channel::bounded::<String>(0);
    let (result_tx, result_rx) = channel::bounded::<Vec<String>>(0);

    let search_handle = thread::spawn(move || {
        search_worker(query_rx, result_tx);
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
