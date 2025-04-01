//! Handles rendering the Tui

mod panels;

use crate::app_manager::*;
use anyhow::Result;
use panels::*;
use ratatui::layout::Layout as RatatuiLayout;
use ratatui::layout::{Constraint::*, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::Frame;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::Stdout;
use std::sync::LazyLock;

pub struct TuiRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Default for TuiRenderer {
    fn default() -> Self {
        let terminal = ratatui::init();
        Self { terminal }
    }
}

impl TuiRenderer {
    pub fn draw_frame(&mut self, state_mngr: &AppManager) -> Result<()> {
        let _result = self
            .terminal
            .draw(|frame| render_callback(frame, state_mngr))?;

        Ok(())
    }
}

struct Layout {
    letters: Rect,
    regex: Rect,
    words: Rect,
}

impl Layout {
    pub fn build(frame: &mut Frame) -> Self {
        let padding = Block::default().padding(Padding::uniform(1));
        let padded_area = padding.inner(frame.area());

        let [top, words] = RatatuiLayout::vertical([Length(3), Fill(1)]).areas(padded_area);
        let [letters, regex] = RatatuiLayout::horizontal([Fill(1), Fill(1)]).areas(top);

        Self {
            letters,
            regex,
            words,
        }
    }
}

/// Handles the layout and rendering of UI components.
fn render_callback(frame: &mut Frame, mngr: &AppManager) {
    let layout = Layout::build(frame);
    let hints = generate_hints(mngr.get_linked_panels());

    LettersInputPanel::new(mngr, &hints).render(frame, layout.letters);
    RegexInputPanel::new(mngr, &hints).render(frame, layout.regex);
    WordsOutputPanel::new(mngr, &hints).render(frame, layout.words);
}

trait Highlight {
    fn highlight(self, state: PanelState) -> Self;
}

impl Highlight for Block<'_> {
    fn highlight(self, state: PanelState) -> Self {
        static RED: LazyLock<Style> = LazyLock::new(|| Style::new().red());
        static YELLOW: LazyLock<Style> = LazyLock::new(|| Style::new().yellow());

        match state {
            PanelState::Default => self,
            PanelState::Selected => self.border_style(*YELLOW),
            PanelState::Error => self.border_style(*RED),
        }
    }
}

impl Direction {
    fn to_char(self) -> char {
        match self {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        }
    }
}

fn generate_hints(linked: HashMap<Direction, PanelRef>) -> HashMap<PanelKind, char> {
    linked
        .into_iter()
        .map(|(dir, panel)| (panel.kind(), dir.to_char()))
        .collect()
}
