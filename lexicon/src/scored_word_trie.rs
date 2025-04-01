use super::word_trie::WordTrie;
use std::cmp;
use std::collections::HashMap;

/// A wrapper over [`WordTrie`] that returns the words together with their scores.
#[derive(Default)]
pub struct ScoredWordTrie {
    pub word_trie: WordTrie,
    pub score_map: HashMap<char, u8>,
}

impl ScoredWordTrie {
    /// Gets all the words that could be built using the given letters sorted by score.
    pub fn get_words(&self, letters: &str) -> Vec<(String, u8)> {
        let words = self.word_trie.get_words(letters);
        let mut words_with_score = words
            .into_iter()
            .map(|word| {
                let score = self.calculate_score(&word);
                (word, score)
            })
            .collect::<Vec<_>>();

        words_with_score.sort_by_key(|(_word, score)| cmp::Reverse(*score));

        words_with_score
    }

    /// Gets all the words that matches the given regular expression sorted by score.
    pub fn get_word_matches(
        &self,
        letters: &str,
        expr: &str,
    ) -> Result<Vec<(String, u8)>, regex::Error> {
        let words = self.word_trie.get_word_matches(letters, expr)?;
        let mut words_with_score = words
            .into_iter()
            .map(|word| {
                let score = self.calculate_score(&word);
                (word, score)
            })
            .collect::<Vec<_>>();

        words_with_score.sort_by_key(|(_word, score)| cmp::Reverse(*score));

        Ok(words_with_score)
    }

    fn calculate_score(&self, word: &str) -> u8 {
        word.chars().filter_map(|ch| self.score_map.get(&ch)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_get_words_with_score() {
        let mut word_trie = WordTrie::default();
        let words = ["rad", "radar", "radical", "radiation", "dart"];
        words.iter().for_each(|word| word_trie.insert(word));
        let word_trie = ScoredWordTrie {
            word_trie,
            score_map: HashMap::from([('r', 1), ('t', 2), ('d', 3)]),
        };

        assert_eq!(
            word_trie
                .get_words("radart")
                .into_iter()
                .collect::<Vec<_>>(),
            [
                ("dart".to_string(), 6u8),
                ("radar".to_string(), 5u8),
                ("rad".to_string(), 4u8),
            ]
        );
    }

    #[test]
    pub fn test_get_word_matches() {
        let mut word_trie = WordTrie::default();
        let words = ["rad", "radar", "radical", "radiation", "dart"];
        words.iter().for_each(|word| word_trie.insert(word));
        let word_trie = ScoredWordTrie {
            word_trie,
            score_map: HashMap::from([('r', 1), ('t', 2), ('d', 3)]),
        };

        assert_eq!(
            word_trie
                .get_word_matches("radart", "^r.*$")
                .expect("a valid regex")
                .into_iter()
                .collect::<Vec<_>>(),
            [("radar".to_string(), 5u8), ("rad".to_string(), 4u8),]
        );
    }
}
