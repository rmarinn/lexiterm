use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Default)]
pub struct WordTrie {
    root: Node,
}

#[derive(Default, PartialEq)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Node {
    /// Append a chain of child nodes and set the last node as a word.
    fn append_word(&mut self, word: &str) {
        let last_node = word.to_lowercase().chars().fold(self, |node, ch| {
            let new_child = node.children.entry(ch).or_default();
            new_child
        });
        last_node.is_word = true;
    }

    /// Create a new [`Path`] starting from this node.
    fn start_path(&self, remaining_letters: HashMap<char, usize>) -> Path {
        Path {
            node: self,
            remaining_letters,
            word_buf: String::new(),
        }
    }
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
        self.root.append_word(word);
    }

    /// Gets all the words that could be built using the given letters.
    pub fn get_words(&self, letters: &str) -> BTreeSet<String> {
        let mut words = BTreeSet::new();

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

        // PERF: we can further optimize this by pruning the duplicate paths that
        // get generated
        while let Some(path) = search_stack.pop_back() {
            if path.node.is_word {
                words.insert(path.word_buf.clone());
            }

            for next_path in step_trie(&path).into_iter() {
                search_stack.push_back(next_path);
            }
        }

        words
    }
}

#[derive(Debug, PartialEq)]
struct Path<'a> {
    node: &'a Node,
    remaining_letters: HashMap<char, usize>,
    word_buf: String,
}

/// Steps through one layer of the Trie using the given letters
fn step_trie<'a>(path: &Path<'a>) -> VecDeque<Path<'a>> {
    let children = &path.node.children;
    let letters = &path.remaining_letters;

    let mut next_paths = VecDeque::new();

    for (ch, _count) in letters.iter() {
        // handle wildcard
        if *ch == '*' {
            let remaining_letters = letters.clone();

            let Ok(remaining_letters) = decrement_count(remaining_letters, ch) else {
                continue;
            };

            for (ch, child) in children.iter() {
                let mut word_buf = path.word_buf.clone();
                word_buf.push(*ch);

                next_paths.push_back(Path {
                    node: child,
                    remaining_letters: remaining_letters.clone(),
                    word_buf,
                });
            }

            continue;
        }

        // handle non-wildcard
        if let Some(child) = children.get(ch) {
            let remaining_letters = letters.clone();

            let Ok(remaining_letters) = decrement_count(remaining_letters, ch) else {
                continue;
            };

            let mut word_buf = path.word_buf.clone();
            word_buf.push(*ch);

            next_paths.push_back(Path {
                node: child,
                remaining_letters,
                word_buf,
            });
        }
    }

    next_paths
}

fn decrement_count(
    mut counts: HashMap<char, usize>,
    ch: &char,
) -> Result<HashMap<char, usize>, ()> {
    let Entry::Occupied(mut ch_entry) = counts.entry(*ch) else {
        return Err(());
    };
    if *ch_entry.get() <= 1 {
        ch_entry.remove();
    } else {
        *ch_entry.get_mut() -= 1;
    }
    Ok(counts)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_append_word_to_node() {
        let mut root = Node::default();

        root.append_word("car");

        let expected = Node {
            children: HashMap::from([(
                'c',
                Node {
                    children: HashMap::from([(
                        'a',
                        Node {
                            children: HashMap::from([(
                                'r',
                                Node {
                                    children: HashMap::new(),
                                    is_word: true,
                                },
                            )]),
                            is_word: false,
                        },
                    )]),
                    is_word: false,
                },
            )]),
            is_word: false,
        };
        assert_eq!(root, expected);
    }

    #[test]
    fn test_step_trie() {
        let mut root = Node::default();
        root.append_word("car");
        root.append_word("cab");

        let initial_path = Path {
            node: &root,
            remaining_letters: HashMap::from([('c', 1), ('a', 1), ('*', 1)]),
            word_buf: "".to_string(),
        };

        let paths = step_trie(&initial_path);

        // assert_eq!(letters, HashMap::from([('a', 1)]));

        let expected_paths = [
            Path {
                node: root.children.get(&'c').unwrap(),
                remaining_letters: HashMap::from([('a', 1), ('*', 1)]),
                word_buf: "c".to_string(),
            },
            Path {
                node: root.children.get(&'c').unwrap(),
                remaining_letters: HashMap::from([('c', 1), ('a', 1)]),
                word_buf: "c".to_string(),
            },
        ];
        for expected_path in expected_paths.iter() {
            assert!(
                paths.contains(expected_path),
                "missing expected path: {:?}\ncalulated paths: {:#?}",
                expected_path,
                paths,
            );
        }
    }

    #[test]
    fn can_insert() {
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

        assert_eq!(
            trie.get_words("radar").into_iter().collect::<Vec<_>>(),
            ["rad", "radar"]
        );
        assert_eq!(
            trie.get_words("radart").into_iter().collect::<Vec<_>>(),
            ["dart", "rad", "radar"]
        );
    }

    #[test]
    pub fn should_handle_wildcards() {
        let mut trie = WordTrie::default();
        let words = ["cam", "cab", "cams", "cabs"];
        words.iter().for_each(|word| trie.insert(word));

        assert_eq!(
            trie.get_words("ca*").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words("*ca").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words("c*a").into_iter().collect::<Vec<_>>(),
            ["cab", "cam"]
        );
        assert_eq!(
            trie.get_words("ca**").into_iter().collect::<Vec<_>>(),
            ["cab", "cabs", "cam", "cams"]
        );
    }
}
