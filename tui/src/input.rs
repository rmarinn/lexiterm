//! Handles user input processing and event polling using `crossterm`.
//!
//! This module listens for terminal events, updates input state, and sends search queries
//! to the worker thread while handling responses.

use anyhow::{anyhow, Result};
use crossbeam::channel::{Receiver, Sender, TrySendError};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

use crate::tui::Tui;

/// Represents different types of input events from the terminal.
///
/// This enum categorizes key events into meaningful actions for the application.
enum InputEvent {
    /// No operation (ignored event).
    NoOp,
    /// Request to exit the application.
    Exit,
    /// Append a character to the input.
    AppendCharToInput(char),
    /// Remove the last character from the input.
    BackSpace,
}

/// Listens for terminal input events and updates the UI accordingly.
///
/// This function continuously listens for key events, processes them, and sends search 
/// queries to the worker thread while updating the terminal UI with results.
///
/// # Arguments
///
/// * `tui` - A reference to the [`Tui`] instance for rendering updates.
/// * `query_tx` - A sender channel to pass search queries to the worker thread.
/// * `result_rx` - A receiver channel to receive processed search results.
///
/// # Errors
///
/// Returns an error if drawing the UI fails or an unexpected issue occurs.
pub fn listen_and_process(
    tui: &Tui,
    query_tx: &Sender<String>,
    result_rx: &Receiver<Vec<String>>,
) -> Result<()> {
    // handle input events
    let mut input_letters = String::new();
    let mut output_words = Vec::new();
    tui.draw_frame(&input_letters, &output_words)?;

    loop {
        // process terminal events
        match process_event(&mut input_letters, query_tx) {
            Ok(true) => break, // exit if requested,
            Ok(false) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Check if the worker thread has responded
        while let Ok(words) = result_rx.try_recv() {
            output_words = words;
        }

        tui.draw_frame(&input_letters, &output_words)?;
    }

    Ok(())
}

/// Polls for keyboard input events and processes them.
///
/// This function batches input events within a short time window for efficiency,
/// preventing excessive query transmissions. If input is modified, the new query
/// is sent to the worker thread.
///
/// # Arguments
///
/// * `input_letters` - A mutable reference to the user’s current input.
/// * `query_tx` - A sender channel for passing search queries.
///
/// # Returns
///
/// * `Ok(true)` - If the user requested to exit.
/// * `Ok(false)` - Otherwise.
///
/// # Errors
///
/// Returns an error if reading input events or sending queries fails.
pub fn process_event(input_letters: &mut String, query_tx: &Sender<String>) -> Result<bool> {
    static POLL_TIMEOUT: Duration = Duration::from_millis(100);
    static BATCH_TIMEOUT: Duration = Duration::from_millis(50);

    let mut input_updated = false;
    let start = Instant::now();

    while start.elapsed() < BATCH_TIMEOUT {
        if event::poll(POLL_TIMEOUT)? {
            let event = event::read()?;
            let event = InputEvent::from(event);

            match event {
                InputEvent::Exit => return Ok(true),
                InputEvent::AppendCharToInput(ch) => {
                    input_letters.push(ch);
                    input_updated = true;
                }
                InputEvent::BackSpace => {
                    input_letters.pop();
                    input_updated = true;
                }
                InputEvent::NoOp => {}
            }
        } else {
            // Exit loop early if there are no more events
            break;
        }
    }

    if input_updated {
        if let Err(err) = query_tx.try_send(input_letters.clone()) {
            match err {
                TrySendError::Full(_) => {}
                TrySendError::Disconnected(err) => {
                    return Err(anyhow!("Worker unexpectedly disconnected: {err}"))
                }
            }
        }
    }

    Ok(false)
}

impl From<crossterm::event::Event> for InputEvent {
    fn from(ev: crossterm::event::Event) -> Self {
        match ev {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Backspace => Self::BackSpace,
                KeyCode::Char(ch) => Self::AppendCharToInput(ch),
                KeyCode::Esc => Self::Exit,
                _ => Self::NoOp,
            },
            _ => Self::NoOp,
        }
    }
}
