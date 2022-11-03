use std::collections::HashMap;

/*
    Trie Nodes
 */
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {children: HashMap::new(), is_word: false}
    }
}

/*
    The actual Trie structure
*/

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {root: TrieNode::new()}
    }

    pub fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for character in word.chars() {
            let next_node = current_node.children.entry(character)
                .or_insert(TrieNode::new());

            current_node = next_node;
        }

        current_node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for character in word.chars() {
            match current_node.children.get(&character) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return current_node.is_word;
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;

        for character in prefix.chars() {
            match current_node.children.get(&character) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return true;
    }
}