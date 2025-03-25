//! Handles terminal-based user interface (TUI) rendering using `ratatui`.
//!
//! This module initializes and manages the terminal UI, rendering the input and output
//! sections dynamically.

use crate::input::{AppState, PanelKind};
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

/// Handles the layout and rendering of UI components.
fn render_callback(frame: &mut Frame, state: &AppState) {
    let padding = Block::default().padding(Padding::uniform(1));
    let padded_area = padding.inner(frame.area());

    let [top, bottom] = Layout::vertical([Length(3), Fill(1)]).areas(padded_area);
    let [input_left, input_right] = Layout::horizontal([Fill(1), Fill(1)]).areas(top);

    let hints = state.panel_mngr.hints();

    let letters_title = hints
        .get(&PanelKind::Letters)
        .map(|hint| format!("Letters ({hint})"))
        .unwrap_or_else(|| "Letters".to_string());
    let letters_block = Block::bordered()
        .title(letters_title)
        .highlight_yellow_if(matches!(state.panel_mngr.selected(), PanelKind::Letters));
    frame.render_widget(
        Paragraph::new(state.letters.as_str()).block(letters_block),
        input_left,
    );

    let regex_title = hints
        .get(&PanelKind::Regex)
        .map(|hint| format!("Regex ({hint})"))
        .unwrap_or_else(|| "Regex".to_string());
    let regex_block = Block::bordered()
        .title(regex_title)
        .highlight_yellow_if(matches!(state.panel_mngr.selected(), PanelKind::Regex))
        .highlight_red_if(state.regex_err.is_some());
    frame.render_widget(
        Paragraph::new(state.regex.as_str()).block(regex_block),
        input_right,
    );

    let words_title = hints
        .get(&PanelKind::Words)
        .map(|hint| format!("Words ({hint})"))
        .unwrap_or_else(|| "Words".to_string());
    let word_block = Block::bordered()
        .title(words_title)
        .highlight_yellow_if(matches!(state.panel_mngr.selected(), PanelKind::Words));
    frame.render_widget(
        Paragraph::new(state.words.join(", "))
            .wrap(Wrap { trim: false })
            .block(word_block),
        bottom,
    );
}

trait HighlightIf {
    fn highlight_yellow_if(self, condition: bool) -> Self;
    fn highlight_red_if(self, condition: bool) -> Self;
}

impl HighlightIf for Block<'_> {
    fn highlight_yellow_if(self, condition: bool) -> Self {
        static YELLOW: LazyLock<Style> = LazyLock::new(|| Style::new().yellow());

        if condition {
            self.border_style(*YELLOW)
        } else {
            self
        }
    }

    fn highlight_red_if(self, condition: bool) -> Self {
        static RED: LazyLock<Style> = LazyLock::new(|| Style::new().red());

        if condition {
            self.border_style(*RED)
        } else {
            self
        }
    }
}
