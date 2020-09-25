#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        println!("Add and Search Words!");
    
        let mut obj = WordDictionary::new();
        
        obj.add_word(String::from("funny"));
        println!("{:?}", obj);

        assert!(obj.search(String::from("fu.ny")));
    }
}



use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    children : HashMap<char, TrieNode>,
    end: bool,
}


impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            end: false
        }
    }
}

#[derive(Debug)]
pub struct WordDictionary {
    root : TrieNode,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        WordDictionary {
            root : TrieNode::new(),
        }
    }
    
    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        WordDictionary::add_to_trie(&word, 0, &mut self.root);
    }

    pub fn add_to_trie(word: &str, index: usize, cur: &mut TrieNode) {
        if index == word.len() {
            cur.end = true;
            return;
        }
    
        let c: char = word.as_bytes()[index] as char; // if you need to get the character as a unicode code point
        let child = cur.children.entry(c).or_insert(TrieNode::new());
        WordDictionary::add_to_trie(word, index+1, child);
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        WordDictionary::search_in_trie(&word, 0, &self.root)
    }

    pub fn search_in_trie(word: &str, index: usize, cur: &TrieNode) -> bool {
        if index == word.len() {
            return cur.end;
        }

        let c: char = word.as_bytes()[index] as char;
        if c == '.' {
            for (_, v) in cur.children.iter() {
                if WordDictionary::search_in_trie(word, index+1, v) {
                    return true;
                }
            }
            return false;
        }
        match cur.children.get(&c) {
            Some(child) => WordDictionary::search_in_trie(word, index+1, child),
            None => false
        }
    }
}


/*
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */