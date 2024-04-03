use std::collections::HashMap;

pub struct Trie {
    root: TrieNode,
}

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    value: String,
    is_word: bool,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::default()
        }
    }

    pub fn insert(&mut self, word: &str) {
        if !word.is_empty() {
            let mut current = &mut self.root;

            for c in word.chars() {
                current = current.children.entry(c).or_insert(TrieNode::default());
            }

            current.value = String::from(word);
            current.is_word = true;

            dbg!(current);
        }
    }

    pub fn find(&self, word: &str) -> Option<String> {
        if !word.is_empty() {
            let mut current = &self.root;

            for c in word.chars() {
                if let Some(trie) = current.children.get(&c) {
                    current = trie;
                }
                else {
                    return None
                }
            }

            return Some(dbg!(current.value.clone()));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_word() {
        let mut t = Trie::new();
        t.insert("hello");
        assert!(!t.root.children.is_empty());
        assert!(!t.root.is_word);

        if let Some(val) = t.root.children.iter().find(|e| e.1.is_word) {
            assert_eq!(val.1.value, "hello");
            assert!(val.1.is_word);
            assert!(val.1.children.is_empty());
            assert_eq!(val.0, &'o');
        }
    }

    #[test]
    fn insert_words() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("hell");
        trie.insert("help");

        assert_eq!(trie.find("hell").unwrap(), "hell");
        assert_eq!(trie.find("hello").unwrap(), "hello");
        assert_eq!(trie.find("help").unwrap(), "help");

    }

    #[test]
    fn new_trie() {
        let t = Trie::new();
        assert!(t.root.children.is_empty());
        assert!(!t.root.is_word);
    }
}