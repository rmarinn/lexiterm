mod node;
mod path;

use node::*;
use path::*;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct WordTrie {
    root: Node,
}

impl WordTrie {
    /// Inserts a words into the Trie
    pub fn insert(&mut self, word: &str) {
        self.root.append_word(word);
    }

    /// Gets all the words that could be built using the given letters.
    pub fn get_words(&self, letters: &str) -> Vec<String> {
        let mut words = Vec::new();

        // Create a frequency map of the available letters
        let letters = letters.chars().fold(HashMap::new(), |mut acc, ch| {
            let Some(ch) = ch.to_lowercase().next() else {
                return acc;
            };
            if ch.is_ascii_alphabetic() || ch == '*' {
                *acc.entry(ch).or_insert(0) += 1;
            }
            acc
        });

        // Build the first search layer
        let start_path = self.root.start_path(letters);
        let mut search_stack = VecDeque::from([start_path]);

        while let Some(path) = search_stack.pop_back() {
            if path.node.is_word {
                words.push(path.word_buf.clone());
            }

            step_trie(&path, &mut search_stack);
        }

        words
    }

    pub fn get_words_sorted(&self, letters: &str) -> Vec<String> {
        let mut words = self.get_words(letters);
        words.sort();
        words
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_insert() {
        let test_word = "radar";
        let mut trie = WordTrie::default();

        trie.insert(test_word);

        let mut current_node = &trie.root;
        for ch in test_word.chars() {
            assert!(
                current_node.children.contains_key(&ch),
                "should have a child `{}` character",
                ch
            );
            current_node = current_node.children.get(&ch).unwrap();
        }
    }

    #[test]
    pub fn test_get_words() {
        let mut trie = WordTrie::default();
        let words = ["rad", "radar", "radical", "radiation", "dart"];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(
            trie.get_words_sorted("radar")
                .into_iter()
                .collect::<Vec<_>>(),
            ["rad", "radar"]
        );
        assert_eq!(
            trie.get_words_sorted("radart")
                .into_iter()
                .collect::<Vec<_>>(),
            ["dart", "rad", "radar"]
        );
    }

    #[test]
    pub fn test_handle_wildcards() {
        let mut trie = WordTrie::default();
        let words = ["cam", "cab", "cams", "cabs"];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(
            trie.get_words_sorted("ca*").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words_sorted("*ca").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words_sorted("c*a").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words_sorted("ca**")
                .into_iter()
                .collect::<Vec<_>>(),
            ["cab", "cabs", "cam", "cams"]
        );
    }
}
