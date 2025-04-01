use super::*;
use ratatui::layout::Rect;

#[derive(Clone, Copy, Default)]
pub enum PanelState {
    #[default]
    Default,
    Selected,
    Error,
}

pub struct LettersInputPanel<'a> {
    title: String,
    state: PanelState,
    letters: &'a str,
}

impl<'a> LettersInputPanel<'a> {
    pub fn new(mngr: &'a AppManager, hints: &'_ HashMap<PanelKind, char>) -> LettersInputPanel<'a> {
        let hint = hints.get(&PanelKind::Letters).copied();
        let title = hint
            .map(|hint| format!("Letters ({hint})"))
            .unwrap_or_else(|| "Letters".to_string());
        let state = (mngr.selected_panel().kind() == PanelKind::Letters)
            .then_some(PanelState::Selected)
            .unwrap_or_default();
        let letters = mngr.get_input_letters();

        Self {
            title,
            state,
            letters,
        }
    }

    pub fn render(self, frame: &mut Frame, rect: Rect) {
        let block = Block::bordered()
            .title(self.title.as_ref())
            .highlight(self.state);
        frame.render_widget(Paragraph::new(self.letters).block(block), rect);
    }
}

pub struct RegexInputPanel<'a> {
    title: String,
    state: PanelState,
    regex: &'a str,
}

impl<'a> RegexInputPanel<'a> {
    pub fn new(mngr: &'a AppManager, hints: &'_ HashMap<PanelKind, char>) -> RegexInputPanel<'a> {
        let hint = hints.get(&PanelKind::Regex).copied();
        let title = hint
            .map(|hint| format!("Regex ({hint})"))
            .unwrap_or_else(|| "Regex".to_string());
        let state = if mngr.is_regex_valid() {
            (mngr.selected_panel().kind() == PanelKind::Regex)
                .then_some(PanelState::Selected)
                .unwrap_or_default()
        } else {
            PanelState::Error
        };
        let regex = mngr.get_input_regex();

        Self {
            title,
            state,
            regex,
        }
    }

    pub fn render(self, frame: &mut Frame, rect: Rect) {
        let block = Block::bordered()
            .title(self.title.as_ref())
            .highlight(self.state);
        frame.render_widget(Paragraph::new(self.regex).block(block), rect);
    }
}

pub struct WordsOutputPanel<'a> {
    title: String,
    state: PanelState,
    words: &'a Vec<String>,
}

impl<'a> WordsOutputPanel<'a> {
    pub fn new(mngr: &'a AppManager, hints: &'_ HashMap<PanelKind, char>) -> WordsOutputPanel<'a> {
        let hint = hints.get(&PanelKind::Words).copied();
        let title = hint
            .map(|hint| format!("Words ({hint})"))
            .unwrap_or_else(|| "Words".to_string());
        let state = (mngr.selected_panel().kind() == PanelKind::Words)
            .then_some(PanelState::Selected)
            .unwrap_or_default();
        let words = mngr.get_ouput_words();

        Self {
            title,
            state,
            words,
        }
    }

    pub fn render(self, frame: &mut Frame, rect: Rect) {
        let block = Block::bordered()
            .title(self.title.as_ref())
            .highlight(self.state);
        frame.render_widget(
            Paragraph::new(self.words.join(", "))
                .wrap(Wrap { trim: false })
                .block(block),
            rect,
        );
    }
}
