//! Handles user input processing and event polling using `crossterm`.
//!
//! This module listens for terminal events, updates input state, and sends search
//! queries to the worker thread while handling responses.

mod panel_manager;

use crate::search_worker::{QueryRequest, QueryResponse};
use crate::tui::Tui;
use anyhow::{anyhow, Result};
use crossbeam::channel::{Receiver, Sender, TrySendError};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

pub use panel_manager::*;

/// Represents different types of input events from the terminal.
///
/// This enum categorizes key events into meaningful actions for the application.
enum InputEvent {
    NoOp,
    Exit,
    AppendCharToInputLetters(char),
    BackSpace,
    SelectPanel(Direction),
}

#[derive(Default)]
pub struct AppState {
    pub letters: String,
    pub regex: String,
    pub regex_err: Option<String>,
    pub words: Vec<String>,
    pub panel_mngr: PanelManager,
}

/// Listens for terminal input events and updates the UI accordingly.
///
/// This function continuously listens for key events, processes them, and sends search
/// queries to the worker thread while updating the terminal UI with results.
pub fn listen_and_process(
    tui: &Tui,
    query_tx: &Sender<QueryRequest>,
    result_rx: &Receiver<Result<QueryResponse>>,
) -> Result<()> {
    // handle input events
    let mut state = AppState::default();
    tui.draw_frame(&state)?;

    loop {
        // process terminal events
        match process_event(&mut state, query_tx) {
            Ok(true) => break, // exit if requested,
            Ok(false) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Check if the worker thread has responded
        while let Ok(query_resp) = result_rx.try_recv() {
            match query_resp {
                Ok(resp) => {
                    state.words = resp.words;
                    state.regex_err = None;
                }
                Err(err) => {
                    state.regex_err = Some(err.to_string());
                }
            }
        }

        tui.draw_frame(&state)?;
    }

    Ok(())
}

impl From<&mut AppState> for QueryRequest {
    fn from(state: &mut AppState) -> Self {
        Self {
            letters: state.letters.clone(),
            regex: state.regex.clone(),
        }
    }
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
pub fn process_event(state: &mut AppState, query_tx: &Sender<QueryRequest>) -> Result<bool> {
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
                InputEvent::AppendCharToInputLetters(ch) => {
                    match state.panel_mngr.selected() {
                        PanelKind::Letters => state.letters.push(ch),
                        PanelKind::Regex => state.regex.push(ch),
                        _ => continue,
                    };
                    input_updated = true;
                }
                InputEvent::BackSpace => {
                    match state.panel_mngr.selected() {
                        PanelKind::Letters => {
                            state.letters.pop();
                        }
                        PanelKind::Regex => {
                            state.regex.pop();
                        }
                        _ => continue,
                    };
                    input_updated = true;
                }
                InputEvent::NoOp => {}
                InputEvent::SelectPanel(direction) => {
                    state.panel_mngr.select_panel(direction);
                }
            }
        } else {
            // Exit loop early if there are no more events
            break;
        }
    }

    if input_updated {
        if let Err(err) = query_tx.try_send(state.into()) {
            match err {
                TrySendError::Full(_) => {}
                TrySendError::Disconnected(err) => {
                    return Err(anyhow!("Worker unexpectedly disconnected: {err:?}"))
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
                KeyCode::Char(ch) => Self::AppendCharToInputLetters(ch),
                KeyCode::Esc => Self::Exit,
                KeyCode::Left => Self::SelectPanel(Direction::Left),
                KeyCode::Right => Self::SelectPanel(Direction::Right),
                KeyCode::Up => Self::SelectPanel(Direction::Up),
                KeyCode::Down => Self::SelectPanel(Direction::Down),
                _ => Self::NoOp,
            },
            _ => Self::NoOp,
        }
    }
}
