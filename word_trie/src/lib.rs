mod word_trie;
mod words;

use std::path::Path;

pub use word_trie::WordTrie;
use words::WordFileReader;

impl WordTrie {
    pub fn new_from_file(path: &Path) -> Self {
        let mut words = WordTrie::default();
        let reader = WordFileReader::new(path);

        for word in reader.into_iter() {
            words.insert(&word);
        }

        words
    }
}
