mod panel_manager;

pub use panel_manager::*;

use regex::Regex;

#[derive(Default)]
pub struct AppManager {
    state: AppState,
}

impl AppManager {
    /// Push a [`char`] to the input field of current panel if it has one and return
    /// `true` if the state was updated.
    pub fn push_ch(&mut self, ch: char) -> bool {
        if ch.is_whitespace() {
            return false;
        }

        match self.state.selected_panel.kind() {
            PanelKind::Letters => self.state.input_letters.push(ch),
            PanelKind::Regex => self.state.input_regex.push(ch),
            PanelKind::Words => return false,
        }

        true
    }

    /// Pops a [`char`] to the input field of current panel if it has one and return
    /// `true` if the state was updated.
    pub fn pop_ch(&mut self) -> bool {
        match self.state.selected_panel.kind() {
            PanelKind::Letters => self.state.input_letters.pop().is_some(),
            PanelKind::Regex => self.state.input_regex.pop().is_some(),
            _ => false,
        }
    }

    pub fn get_input_letters(&self) -> &str {
        &self.state.input_letters
    }

    pub fn get_input_regex(&self) -> &str {
        &self.state.input_regex
    }

    pub fn get_ouput_words(&self) -> &Vec<String> {
        &self.state.output_words
    }

    pub fn set_output_words(&mut self, words: Vec<String>) {
        self.state.output_words = words;
    }

    pub fn is_letters_valid(&self) -> bool {
        self.state.input_letters.len() > 1
    }

    pub fn is_regex_valid(&self) -> bool {
        let expr = &self.state.input_regex;
        expr.is_empty() || Regex::new(expr).is_ok()
    }
}

pub struct AppState {
    input_letters: String,
    input_regex: String,
    output_words: Vec<String>,
    selected_panel: PanelRef,
}

impl Default for AppState {
    fn default() -> Self {
        let letters = PanelRef::new(PanelKind::Letters);
        let regex = PanelRef::new(PanelKind::Regex);
        let words = PanelRef::new(PanelKind::Words);

        letters.link(Direction::Right, regex.clone());
        letters.link(Direction::Down, words.clone());
        regex.link(Direction::Left, letters.clone());
        regex.link(Direction::Down, words.clone());

        Self {
            selected_panel: letters,
            input_letters: String::new(),
            input_regex: String::new(),
            output_words: Vec::new(),
        }
    }
}
