//! Handles user input processing and event polling using `crossterm`.
//!
//! This module listens for terminal events, updates input state, and sends search
//! queries to the worker thread while handling responses.

use crate::app_manager::*;
use crate::search_worker::{QueryRequest, QueryResponse};
use crate::tui_renderer::*;
use anyhow::{anyhow, Result};
use crossbeam::channel::{Receiver, Sender, TrySendError};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

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

/// Listens for terminal input events and updates the UI accordingly.
///
/// This function continuously listens for key events, processes them, and sends search
/// queries to the worker thread while updating the terminal UI with results.
pub fn listen_and_process(
    mut mngr: AppManager,
    mut renderer: TuiRenderer,
    query_tx: &Sender<QueryRequest>,
    result_rx: &Receiver<QueryResponse>,
) -> Result<()> {
    // handle input events
    renderer.draw_frame(&mngr)?;

    loop {
        if mngr.process_event(query_tx)? {
            // exit if requested,
            break;
        }

        // Check if the worker thread has responded
        if let Some(resp) = result_rx.try_recv().into_iter().last() {
            mngr.set_output_words(resp.words);
        }

        renderer.draw_frame(&mngr)?;
    }

    Ok(())
}

impl AppManager {
    /// Poll for keyboard input events, processes them, then return `true` if an exit
    /// signal was received.
    ///
    /// This function batches input events within a short time window for efficiency,
    /// preventing excessive query transmissions. If input is modified, the new query
    /// is sent to the worker thread.
    pub fn process_event(&mut self, query_tx: &Sender<QueryRequest>) -> Result<bool> {
        static POLL_TIMEOUT: Duration = Duration::from_millis(100);
        static BATCH_TIMEOUT: Duration = Duration::from_millis(50);

        let mut input_updated = false;
        let start = Instant::now();

        while start.elapsed() < BATCH_TIMEOUT {
            if !event::poll(POLL_TIMEOUT)? {
                break;
            }

            let event = event::read()?;
            let event = InputEvent::from(event);

            input_updated = match event {
                InputEvent::Exit => return Ok(true),
                InputEvent::AppendCharToInputLetters(ch) => self.push_ch(ch),
                InputEvent::BackSpace => self.pop_ch(),
                InputEvent::NoOp => false,
                InputEvent::SelectPanel(direction) => {
                    self.select_panel(direction);
                    false
                }
            };
        }

        // Send inputs to worker
        if !input_updated {
            return Ok(false);
        }

        if !self.is_letters_valid() || !self.is_regex_valid() {
            return Ok(false);
        }

        if let Err(err) = query_tx.try_send(QueryRequest {
            letters: self.get_input_letters().into(),
            regex: self.get_input_regex().into(),
        }) {
            match err {
                TrySendError::Full(_) => {}
                TrySendError::Disconnected(err) => {
                    return Err(anyhow!("Worker unexpectedly disconnected: {err:?}"))
                }
            }
        }

        Ok(false)
    }
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
