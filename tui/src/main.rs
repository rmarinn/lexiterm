use std::cmp;
use std::error::Error;
use std::path::Path;
use std::sync::{LazyLock, OnceLock, RwLock};

use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
use ratatui::Frame;
use word_trie::WordTrie;

static WORDS: OnceLock<WordTrie> = OnceLock::new();
static INPUT_LETTERS: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new(String::new()));
static OUTPUT_WORDS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(Vec::new()));

fn main() {
    if let Err(_word_trie) = WORDS.set(WordTrie::new_from_file(Path::new("../words.txt"))) {
        eprintln!("failed to initialize Words trie");
        return;
    }

    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw_frame).expect("failed to draw frame");

        match handle_event() {
            Ok(exit) => {
                if exit {
                    break;
                }
            }
            Err(err) => {
                eprintln!("failed to read event: {err}");
                break;
            }
        }
    }

    ratatui::restore();
}

/// Handles an [`Event`] and returns `Ok(true)` if an exit signal was received.
///
/// [`Event`]: crossterm::event::Event
fn handle_event() -> Result<bool, Box<dyn Error>> {
    let event = event::read()?;

    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char(ch) => {
                let letters = {
                    // update the input letters
                    let mut letters_lock = INPUT_LETTERS.write().expect("acquire write lock");
                    letters_lock.push(ch);
                    letters_lock.clone()
                };

                calculate_words(&letters);
            }
            KeyCode::Backspace => {
                let mut letters = INPUT_LETTERS.write().expect("acquire write lock");
                letters.pop();
                calculate_words(&letters);
            }
            KeyCode::Esc => return Ok(true),
            _ => {}
        },
        Event::Paste(_) => todo!("handle pasting"),
        Event::Resize(_, _) => todo!("handle resizing"),
        _ => {}
    }

    Ok(false)
}

fn calculate_words(letters: &str) {
    if letters.len() > 1 {
        let mut words = WORDS.get().unwrap().get_words(letters);
        words.sort_by_key(|word| cmp::Reverse(word.len()));
        {
            let mut words_lock = OUTPUT_WORDS.write().expect("acquire write lock");
            *words_lock = words;
        }
    } else {
        {
            let mut words_lock = OUTPUT_WORDS.write().expect("acquire write lock");
            words_lock.clear();
        }
    }
}

fn draw_frame(frame: &mut Frame) {
    use Constraint::*;

    let padding = Block::default().padding(Padding::uniform(1));
    let padded_area = padding.inner(frame.area());

    let [input_area, output_area] = Layout::vertical([Length(3), Fill(1)]).areas(padded_area);
    let [input_left, input_right] = Layout::horizontal([Length(10), Fill(1)]).areas(input_area);

    let input_letters = { INPUT_LETTERS.read().expect("acquire read lock").clone() };
    let output_words = { OUTPUT_WORDS.read().expect("acquire read lock").clone() };

    frame.render_widget(
        Paragraph::new("Letters:").block(Block::default().padding(Padding::vertical(1))),
        input_left,
    );
    frame.render_widget(
        Paragraph::new(input_letters).block(Block::bordered()),
        input_right,
    );
    frame.render_widget(
        Paragraph::new(output_words.join(" "))
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title("words")),
        output_area,
    );
}
