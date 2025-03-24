use super::Path;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default, PartialEq)]
pub struct Node {
    pub children: HashMap<char, Node>,
    pub is_word: bool,
}

impl Node {
    /// Append a chain of child nodes and set the last node as a word.
    pub fn append_word(&mut self, word: &str) {
        let last_node = word.to_lowercase().chars().fold(self, |node, ch| {
            let new_child = node.children.entry(ch).or_default();
            new_child
        });
        last_node.is_word = true;
    }

    /// Create a new [`Path`] starting from this node.
    pub fn start_path(&self, remaining_letters: HashMap<char, usize>) -> Path {
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_append_word_to_node() {
        let mut root = Node::default();

        root.append_word("car");

        let expected = Node {
            is_word: false,
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
        };
        assert_eq!(root, expected);
    }
}
