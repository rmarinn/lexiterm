//! Handles terminal-based user interface (TUI) rendering using `ratatui`.
//!
//! This module initializes and manages the terminal UI, rendering the input and output
//! sections dynamically.

use crate::input::{AppState, InputField};
use anyhow::Result;
use ratatui::layout::{Constraint::*, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use std::cell::RefCell;
use std::io::Stdout;
use std::sync::LazyLock;

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
    pub fn draw_frame(&self, state: &AppState) -> Result<()> {
        let _result = self
            .terminal
            .borrow_mut()
            .draw(|frame| render_callback(frame, state))?;

        Ok(())
    }
}

trait HighlightIf {
    fn highlight_if(self, condition: bool) -> Self;
}

impl HighlightIf for Block<'_> {
    fn highlight_if(self, condition: bool) -> Self {
        static YELLOW: LazyLock<Style> = LazyLock::new(|| Style::new().yellow());

        if condition {
            self.border_style(*YELLOW)
        } else {
            self
        }
    }
}

/// Handles the layout and rendering of UI components.
fn render_callback(frame: &mut Frame, state: &AppState) {
    let padding = Block::default().padding(Padding::uniform(1));
    let padded_area = padding.inner(frame.area());

    let [top, bottom] = Layout::vertical([Length(3), Fill(1)]).areas(padded_area);
    let [input_left, input_right] = Layout::horizontal([Fill(1), Fill(1)]).areas(top);

    let letters_block = Block::bordered()
        .title("Letters (←)")
        .highlight_if(matches!(state.input_field, InputField::Letters));
    frame.render_widget(
        Paragraph::new(state.letters.as_str()).block(letters_block),
        input_left,
    );

    let regex_title = state.regex_err.as_ref().map_or_else(
        || "Regex (→)".to_string(),
        |err| format!("Regex (err: {err})"),
    );
    let regex_block = Block::bordered()
        .title(regex_title)
        .highlight_if(matches!(state.input_field, InputField::Regex));
    frame.render_widget(
        Paragraph::new(state.regex.as_str()).block(regex_block),
        input_right,
    );

    frame.render_widget(
        Paragraph::new(state.words.join(", "))
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title("words")),
        bottom,
    );
}
