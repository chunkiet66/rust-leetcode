#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution_basic() {
        let words = vec!(String::from("oath"), String::from("pea"), String::from("eat"), String::from("rain"));
        let board = vec!(
            vec!('o','a','a','n'),
            vec!('e','t','a','e'),
            vec!('i','h','k','r'),
            vec!('i','f','l','v'),
        );
        let expected = vec!(String::from("eat"),String::from("oath"));
        let output = SolutionBasic::find_words(board, words);
        assert!(verify_output(expected, output));

        let words : Vec<String> = ["ab","cb","ad","bd","ac","ca","da","bc","db","adcb","dabc","abb","acb"].iter().map(|&s| s.into()).collect();
        let board = vec!(
            vec!('a','b'),
            vec!('c','d'),
        );
        let expected : Vec<String> = ["ab","ac","bd","ca","db"].iter().map(|&s| s.into()).collect();
        let output = SolutionBasic::find_words(board, words);
        assert!(verify_output(expected, output));

        let words : Vec<String> = ["aba","baa","bab","aaab","aaa","aaaa","aaba"].iter().map(|&s| s.into()).collect();
        let board = vec!(
            vec!('a','b'),
            vec!('a','a'),
        );
        let expected : Vec<String> = ["aaa","aaab","aaba","aba","baa"].iter().map(|&s| String::from(s)).collect();
        let output = SolutionBasic::find_words(board, words);
        assert!(verify_output(expected, output));
    }

    #[test]
    fn test_solution_trie() {
        let words = vec!(String::from("oath"), String::from("pea"), String::from("eat"), String::from("rain"));
        let board = vec!(
            vec!('o','a','a','n'),
            vec!('e','t','a','e'),
            vec!('i','h','k','r'),
            vec!('i','f','l','v'),
        );
        let expected = vec!(String::from("eat"),String::from("oath"));
        let output = SolutionTrie::find_words(board, words);
        println!("hello {:?}", output);
        assert!(verify_output(expected, output));

        let words : Vec<String> = ["ab","cb","ad","bd","ac","ca","da","bc","db","adcb","dabc","abb","acb"].iter().map(|&s| s.into()).collect();
        let board = vec!(
            vec!('a','b'),
            vec!('c','d'),
        );
        let expected : Vec<String> = ["ab","ac","bd","ca","db"].iter().map(|&s| s.into()).collect();
        let output = SolutionTrie::find_words(board, words);
        assert!(verify_output(expected, output));

        let words : Vec<String> = ["aba","baa","bab","aaab","aaa","aaaa","aaba"].iter().map(|&s| s.into()).collect();
        let board = vec!(
            vec!('a','b'),
            vec!('a','a'),
        );
        let expected : Vec<String> = ["aaa","aaab","aaba","aba","baa"].iter().map(|&s| String::from(s)).collect();
        let output = SolutionTrie::find_words(board, words);
        assert!(verify_output(expected, output));
    }

    pub fn verify_output(expected: Vec<String>, output: Vec<String>) -> bool {
        if expected.len() != output.len() {
            return false;
        }
        for expect in expected.iter() {
            if !output.contains(expect) {
                return false;
            }
        }
        true
    }
}

use std::collections::HashMap;

pub struct Trie {
    word: Option<String>,
    children : HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            word: None,
            children : HashMap::new(),
        }
    }
}

pub struct SolutionTrie {
}

impl SolutionTrie {

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut res : Vec<String> = Vec::new();
        if board.is_empty() {
            return res;
        }
        let mut root = Trie::new();
        for word in words {
            SolutionTrie::build_trie(&word, 0, &mut root.children);
        }
        let mut seen : Vec<Vec<bool>> = vec!(vec!(false; board[0].len()); board.len());

        for r in 0..board.len() {
            for c in 0..board[0].len() {
                SolutionTrie::dfs_trie(&board, &mut seen, r as i32, c as i32, &mut root, &mut res);
            }
        }
        res
    }

    fn build_trie(word: &str, index: usize, children: &mut HashMap<char, Trie>) {
        let key = word.as_bytes()[index] as char;
        let child = children.entry(key).or_insert(Trie::new());
        if index == word.len()-1 {
            child.word = Some(String::from(word));
            return;
        }
        SolutionTrie::build_trie(word, index+1, &mut child.children);
    }

    fn dfs_trie(board: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, r: i32, c: i32, parent: &mut Trie, res: &mut Vec<String>) {
        let ri = r as usize;
        let ci = c as usize;
        if r < 0 || c < 0 || ri >= board.len() || ci >= board[0].len() {
            return;
        }
        if seen[ri][ci] {
            return;
        }

        seen[ri][ci] = true;
        let key = board[ri][ci];
        if let Some(trie) = parent.children.get_mut(&key) {
            if let Some(word) = &trie.word {
                res.push(String::from(word));
                trie.word = None;
            }
            SolutionTrie::dfs_trie(board, seen, r+1, c, trie, res);
            SolutionTrie::dfs_trie(board, seen, r, c+1, trie, res);
            SolutionTrie::dfs_trie(board, seen, r-1, c, trie, res);
            SolutionTrie::dfs_trie(board, seen, r, c-1, trie, res);

            // Optimization: incrementally remove the leaf nodes
            if trie.children.is_empty() {
                parent.children.remove(&key);
            }
        }
        seen[ri][ci] = false;
    }
}

pub struct SolutionBasic {
}

impl SolutionBasic {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut res : Vec<String> = Vec::new();
        if board.len() == 0 {
            return res;
        }
        let mut seen: Vec<Vec<bool>> = vec!(vec!(false; board[0].len()); board.len());
        for word in words {
            println!("word: {}", word);
            'outer: for r in 0..board.len() {
                for c in 0..board[0].len() {
                    if SolutionBasic::dfs(&board, &word, &mut seen, r as i32, c as i32, 0) {
                        res.push(word);
                        break 'outer;
                    }
                }
            }
        }
        res
    }

    pub fn dfs(board: &Vec<Vec<char>>, word: &str,
                seen: &mut Vec<Vec<bool>>, r: i32, c: i32, index: usize) -> bool {
        if index == word.len() {
            return true;
        }
        if r < 0 || c < 0 || r as usize >= board.len() || c as usize >= board[0].len() {
            return false;
        }

        if seen[r as usize][c as usize] || (word.as_bytes()[index] as char != board[r as usize][c as usize]) {
            return false
        }

        seen[r as usize][c as usize] = true;

        let ret = SolutionBasic::dfs(board, word, seen, r+1, c, index+1) ||
                    SolutionBasic::dfs(board, word, seen, r, c+1, index+1) ||
                    SolutionBasic::dfs(board, word, seen, r-1, c, index+1) ||
                    SolutionBasic::dfs(board, word, seen, r, c-1, index+1);

        seen[r as usize][c as usize] = false;
        ret
    }
}