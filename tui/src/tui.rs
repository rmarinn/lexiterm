//! Handles terminal-based user interface (TUI) rendering using `ratatui`.
//!
//! This module initializes and manages the terminal UI, rendering the input and output
//! sections dynamically.

use anyhow::Result;
use ratatui::layout::{Constraint::*, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::cell::RefCell;
use std::io::Stdout;

/// A wrapper around `ratatui`'s [`Terminal`] to manage TUI rendering.
///
/// This struct encapsulates the terminal state and provides methods to
/// update the UI.
pub struct Tui {
    terminal: RefCell<Terminal<CrosstermBackend<Stdout>>>,
}

impl Default for Tui {
    /// Creates a new [`Tui`] instance with default terminal initialization.
    fn default() -> Self {
        let terminal = ratatui::init().into();
        Self { terminal }
    }
}

impl Tui {
    /// Renders the UI frame with the given input and output data.
    ///
    /// This function updates the terminal interface, displaying:
    /// - The current input letters
    /// - The processed output words
    ///
    /// # Arguments
    ///
    /// * `input_letters` - The string containing user-typed characters.
    /// * `output_words` - A list of words resulting from processing `input_letters`.
    ///
    /// # Errors
    ///
    /// Returns an error if the terminal rendering operation fails.
    pub fn draw_frame(&self, input_letters: &str, output_words: &[String]) -> Result<()> {
        let _result = self
            .terminal
            .borrow_mut()
            .draw(|frame| render_callback(frame, input_letters, output_words))?;

        Ok(())
    }
}

/// Handles the layout and rendering of UI components.
///
/// This function is responsible for drawing the UI frame, which consists of:
/// - **An input section** (label and text box for user-typed characters)
/// - **An output section** (displaying the processed words)
///
/// # Arguments
///
/// * `frame` - The [`Frame`] to render UI components onto.
/// * `input_letters` - The current user input string.
/// * `output_words` - The list of words generated based on input.
fn render_callback(frame: &mut Frame, input_letters: &str, output_words: &[String]) {
    let padding = Block::default().padding(Padding::uniform(1));
    let padded_area = padding.inner(frame.area());

    let [input_area, output_area] = Layout::vertical([Length(3), Fill(1)]).areas(padded_area);
    let [input_left, input_right] = Layout::horizontal([Length(10), Fill(1)]).areas(input_area);

    frame.render_widget(
        Paragraph::new("Letters:").block(Block::default().padding(Padding::vertical(1))),
        input_left,
    );
    frame.render_widget(
        Paragraph::new(input_letters).block(Block::bordered()),
        input_right,
    );
    frame.render_widget(
        Paragraph::new(output_words.join(", "))
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title("words")),
        output_area,
    );
}
