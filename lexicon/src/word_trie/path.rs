use super::Node;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Path<'a> {
    pub node: &'a Node,
    pub remaining_letters: HashMap<char, usize>,
    pub word_buf: String,
}

/// Steps through one layer of the Trie using the given letters and return
/// the next possible paths
pub fn step_trie<'a>(path: &Path<'a>, search_stack: &mut VecDeque<Path<'a>>) {
    let children = &path.node.children;
    let letters = &path.remaining_letters;

    for ch in letters.keys() {
        // handle wildcard
        if *ch == '*' {
            let remaining_letters = letters.clone();

            let Ok(remaining_letters) = decrement_count(remaining_letters, ch) else {
                continue;
            };

            for (ch, child) in children
                .iter()
                .filter(|c| !remaining_letters.contains_key(c.0))
            {
                let mut word_buf = path.word_buf.clone();
                word_buf.push(*ch);

                search_stack.push_back(Path {
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

            search_stack.push_back(Path {
                node: child,
                remaining_letters,
                word_buf,
            });
        }
    }
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

        let mut paths = VecDeque::new();
        step_trie(&initial_path, &mut paths);

        let expected_paths = [Path {
            node: root.children.get(&'c').unwrap(),
            remaining_letters: HashMap::from([('a', 1), ('*', 1)]),
            word_buf: "c".to_string(),
        }];
        for expected_path in expected_paths.iter() {
            assert!(
                paths.contains(expected_path),
                "missing expected path: {:?}\ncalulated paths: {:#?}",
                expected_path,
                paths,
            );
        }
    }
}
