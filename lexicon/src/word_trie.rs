mod node;
mod path;

use node::*;
use path::*;
use regex::Regex;
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

        let letters_map = to_frequency_map(letters);

        // Prepare the first search layer
        let start_path = self.root.start_path(letters_map);
        let mut search_stack = VecDeque::from([start_path]);

        // BFS
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

    /// Gets all the words that matches the given regular expression.
    pub fn get_word_matches(&self, letters: &str, expr: &str) -> Result<Vec<String>, regex::Error> {
        let mut words = Vec::new();

        let re = Regex::new(expr)?;

        let letters_map = to_frequency_map(letters);

        // Prepare the first search layer
        let start_path = self.root.start_path(letters_map);
        let mut search_stack = VecDeque::from([start_path]);

        // BFS
        while let Some(path) = search_stack.pop_back() {
            if path.node.is_word && re.is_match(&path.word_buf) {
                words.push(path.word_buf.clone());
            }

            step_trie(&path, &mut search_stack);
        }

        Ok(words)
    }

    pub fn get_word_matches_sorted(
        &self,
        letters: &str,
        expr: &str,
    ) -> Result<Vec<String>, regex::Error> {
        let mut words = self.get_word_matches(letters, expr)?;
        words.sort();
        Ok(words)
    }
}

/// Creates a frequency map of the available letters
fn to_frequency_map(letters: &str) -> HashMap<char, usize> {
    let letters = letters.chars().fold(HashMap::new(), |mut acc, ch| {
        let Some(ch) = ch.to_lowercase().next() else {
            return acc;
        };
        if ch.is_ascii_alphabetic() || ch == '*' {
            *acc.entry(ch).or_insert(0) += 1;
        }
        acc
    });

    letters
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

        assert_eq!(trie.get_words_sorted("radar"), ["rad", "radar"]);
        assert_eq!(trie.get_words_sorted("radart"), ["dart", "rad", "radar"]);
    }

    #[test]
    pub fn test_handle_wildcards() {
        let mut trie = WordTrie::default();
        let words = ["cam", "cab", "cams", "cabs"];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(trie.get_words_sorted("ca*"), ["cab", "cam"]);
        assert_eq!(trie.get_words_sorted("*ca"), ["cab", "cam"]);
        assert_eq!(trie.get_words_sorted("c*a"), ["cab", "cam"]);
        assert_eq!(
            trie.get_words_sorted("ca**"),
            ["cab", "cabs", "cam", "cams"]
        );
    }

    #[test]
    pub fn test_get_words_filtered() {
        let mut trie = WordTrie::default();
        let words = [
            "carbon", "car", "dart", "cam", "cart", "fart", "crime", "com", "rad", "radar",
        ];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(
            trie.get_word_matches_sorted("carbont", "car.*")
                .expect("a valid regex"),
            ["car", "carbon", "cart"]
        );

        assert_eq!(
            trie.get_word_matches_sorted("cartf", ".*art")
                .expect("a valid regex"),
            ["cart", "fart"]
        );

        assert_eq!(
            trie.get_word_matches_sorted("crimea", "c.*m")
                .expect("a valid regex"),
            ["cam", "crime"]
        );

        assert_eq!(
            trie.get_word_matches_sorted("crimea*", "c.{1}m")
                .expect("a valid regex"),
            ["cam", "com"]
        );

        assert_eq!(
            trie.get_word_matches_sorted("radart", "^r.*$")
                .expect("a valid regex"),
            ["rad", "radar"]
        );
    }
}
