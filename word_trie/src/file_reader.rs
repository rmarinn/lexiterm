use super::*;
use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct FileReader {
    reader: BufReader<File>,
}

/// Reads words from a file.
///
/// # Usage
///
/// This reader the file line by line
///
/// ```ignore
/// use std::path::Path;
/// use file_reader::WordFileReader;
///
/// let reader = WordFileReader::new(Path::new("./words.txt")).unwrap();
///
/// for line in reader.into_iter() {
///     println!("{line}");
/// }
/// ```
impl FileReader {
    fn new(path: &Path) -> Result<Self, OpenFileError> {
        let file = File::open(path).map_err(|err| OpenFileError {
            path: path.to_string_lossy().to_string(),
            err,
        })?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    /// Returns an iterator of strings over a words file.
    ///
    /// # Example Words File
    ///
    /// ```txt
    /// aam
    /// aani
    /// aardvark
    /// aardvarks
    /// aardwolf
    /// aardwolves
    /// ```
    pub fn parse_word_file(path: &Path) -> Result<Vec<String>, ParseWordFileError> {
        let reader = FileReader::new(path)?;

        let mut words = Vec::new();
        for word in reader.into_iter() {
            if word.chars().any(|ch| !ch.is_ascii_alphabetic()) {
                return Err(ParseWordFileError::InvalidWord(word));
            }
            words.push(word);
        }

        Ok(words)
    }

    /// Returns an iterator of strings over a words file.
    ///
    /// # Example Words File
    ///
    /// ```txt
    /// a=1
    /// b=3
    /// c=3
    /// d=2
    /// e=1
    /// f=4
    /// ```
    pub fn parse_scores_file(path: &Path) -> Result<HashMap<char, u8>, ParseScoreFileError> {
        let mut scores = HashMap::new();
        let reader = FileReader::new(path)?;

        for (i, line_str) in reader.into_iter().enumerate() {
            let (ch_str, score_str) = line_str
                .split_once('=')
                .ok_or(ParseScoreFileError::MissingEqualSign(i, line_str.clone()))?;

            if ch_str.len() != 1 {
                return Err(ParseScoreFileError::InvalidChar(ch_str.to_string()));
            }

            let ch = ch_str.chars().next().unwrap();
            if !ch.is_ascii_alphabetic() {
                return Err(ParseScoreFileError::InvalidChar(ch_str.to_string()));
            }

            let score = score_str
                .parse::<u8>()
                .map_err(|err| ParseScoreFileError::InvalidScore(score_str.to_string(), err))?;

            scores.insert(ch, score);
        }

        Ok(scores)
    }
}

impl IntoIterator for FileReader {
    type Item = String;
    type IntoIter = FileLineIterator;

    fn into_iter(self) -> Self::IntoIter {
        FileLineIterator {
            reader: self.reader,
        }
    }
}

pub struct FileLineIterator {
    reader: BufReader<File>,
}

impl Iterator for FileLineIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut word = String::new();
        let len = self
            .reader
            .read_line(&mut word)
            .expect("read line from file");

        if len == 0 {
            None
        } else {
            // Trim is done to remove the newline character at the end
            Some(word.trim_end().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_load_words_from_file() {
        let expected_words = vec![
            "a", "aa", "aaa", "aah", "aahed", "aahing", "aahs", "aal", "aalii", "aaliis",
        ];
        let n_words = expected_words.len();
        let mut words = Vec::with_capacity(n_words);
        let reader =
            FileReader::new(Path::new("../words.txt")).expect("should load words from file");

        for (n, word) in reader.into_iter().enumerate() {
            words.push(word);
            if n + 1 == n_words {
                break;
            }
        }

        assert_eq!(words, expected_words)
    }
}
