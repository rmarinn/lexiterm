use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Default)]
pub struct WordTrie {
    root: Node,
}

#[derive(Default)]
pub struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children = self.children.keys();
        write!(f, "{{is_word: {}, children: {:?}}}", self.is_word, children)
    }
}

impl WordTrie {
    /// Inserts a words into the Trie
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.to_lowercase().chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_word = true;
    }

    /// Gets all the words that could be built using the given letters.
    pub fn get_words(&self, letters: &str) -> Vec<String> {
        let mut words = Vec::new();
        let mut search_stack = VecDeque::new();

        // Create a frequency map of the available letters
        let letters = letters.chars().fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        });

        // Build the first search layer
        for (ch, count) in letters.iter() {
            if let Some(child) = self.root.children.get(ch) {
                let mut remaining_letters = letters.clone();

                if *count == 1 {
                    remaining_letters.remove(ch);
                } else {
                    *remaining_letters.get_mut(ch).unwrap() -= 1;
                }

                search_stack.push_back(SearchBranch {
                    node: child,
                    remaining_letters,
                    word_buf: ch.to_string(),
                });
            }
        }

        while let Some(branch) = search_stack.pop_back() {
            if branch.node.is_word {
                words.push(branch.word_buf.clone());
            }

            for (ch, count) in branch.remaining_letters.iter() {
                if let Some(child) = branch.node.children.get(ch) {
                    // Prepare remaining letters for the next branch
                    let mut remaining_letters = branch.remaining_letters.clone();
                    if *count == 1 {
                        remaining_letters.remove(ch);
                    } else {
                        *remaining_letters.get_mut(ch).unwrap() -= 1;
                    }

                    // Prepare word buffer for the next branch
                    let mut word_buf = branch.word_buf.clone();
                    word_buf.push(*ch);

                    search_stack.push_back(SearchBranch {
                        node: child,
                        remaining_letters,
                        word_buf,
                    });
                }
            }
        }

        words
    }

    /// Gets all the words that could be built using the given letters, sorted in alphabetical
    /// order.
    pub fn get_words_sorted(&self, letters: &str) -> Vec<String> {
        let mut words = self.get_words(letters);
        words.sort();
        words
    }
}

#[derive(Debug)]
struct SearchBranch<'a> {
    node: &'a Node,
    remaining_letters: HashMap<char, usize>,
    word_buf: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn can_insert() {
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
    pub fn can_get_words() {
        let mut trie = WordTrie::default();
        let words = ["rad", "radar", "radical", "radiation", "dart"];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(trie.get_words_sorted("radar"), ["rad", "radar"]);
        assert_eq!(trie.get_words_sorted("radart"), ["dart", "rad", "radar"]);
    }
}
