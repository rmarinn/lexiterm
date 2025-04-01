mod file_reader;
mod scored_word_trie;
mod word_trie;

use file_reader::FileReader;
use std::{num::ParseIntError, path::Path};
use thiserror::Error;

pub use scored_word_trie::ScoredWordTrie;
pub use word_trie::WordTrie;

impl WordTrie {
    pub fn new_from_file(path: &Path) -> Result<Self, ParseFileError> {
        let mut word_trie = WordTrie::default();
        let words = FileReader::parse_word_file(path)?;

        for word in words.into_iter() {
            word_trie.insert(&word);
        }

        Ok(word_trie)
    }
}

impl ScoredWordTrie {
    pub fn new_from_files(words_path: &Path, scores_path: &Path) -> Result<Self, ParseFileError> {
        let word_trie = WordTrie::new_from_file(words_path)?;
        let score_map = FileReader::parse_scores_file(scores_path)?;

        Ok(Self {
            word_trie,
            score_map,
        })
    }
}

#[derive(Debug, Error)]
pub enum ParseFileError {
    #[error(transparent)]
    ParseWordFile(#[from] ParseWordFileError),
    #[error(transparent)]
    ParseScoreFile(#[from] ParseScoreFileError),
}

#[derive(Debug, Error)]
#[error("failed to load word file from `{path}`: {err}")]
pub struct OpenFileError {
    path: String,
    err: std::io::Error,
}

#[derive(Debug, Error)]
pub enum ParseWordFileError {
    #[error(transparent)]
    OpenFile(#[from] OpenFileError),
    #[error("Invalid word: \"{0}\". Words can only contain characters between a-z or A-Z.")]
    InvalidWord(String),
}

#[derive(Debug, Error)]
pub enum ParseScoreFileError {
    #[error(transparent)]
    OpenFile(#[from] OpenFileError),
    #[error("Line {0} is missing an equal sign `=`: {1}")]
    MissingEqualSign(usize, String),
    #[error("The left side of the equal sign `=` must be a single character, got: {0}.")]
    InvalidChar(String),
    #[error(
        "The right side of the equal sign `=` must be a valid score but got `{0}`: error: {1}"
    )]
    InvalidScore(String, ParseIntError),
}
