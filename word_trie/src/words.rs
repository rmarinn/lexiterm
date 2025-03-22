use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub struct WordFileReader {
    reader: BufReader<File>,
}

impl WordFileReader {
    pub fn new(path: &Path) -> Self {
        let file = File::open(path).expect("open file");
        let reader = BufReader::new(file);
        Self { reader }
    }
}

impl IntoIterator for WordFileReader {
    type Item = String;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            reader: self.reader,
        }
    }
}

pub struct IntoIter {
    reader: BufReader<File>,
}

impl Iterator for IntoIter {
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

    #[test]
    fn can_load_words_from_file() {
        let expected_words = vec![
            "a", "aa", "aaa", "aah", "aahed", "aahing", "aahs", "aal", "aalii", "aaliis",
        ];
        let n_words = expected_words.len();
        let mut words = Vec::with_capacity(n_words);
        let reader = WordFileReader::new(Path::new("../words.txt"));

        for (n, word) in reader.into_iter().enumerate() {
            words.push(word);
            if n + 1 == n_words {
                break;
            }
        }

        assert_eq!(words, expected_words,)
    }
}
