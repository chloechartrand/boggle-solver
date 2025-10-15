#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::collections::HashMap;

/*
    Fill in the boggle function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.

    To demonstrate how the HashMap can be used to store word/coord associations,
    the function stub below contains two sample words added from the 2x2 board.
*/
fn boggle(board: &[&str], words: &[String]) -> HashMap<String, Vec<(u8, u8)>> {
    let mut found: HashMap<String, Vec<(u8, u8)>> = HashMap::new();
    let mut trie = Trie::new(words);

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let mut visited = vec![vec![false; board[0].len()]; board.len()];
            let mut word = String::new();
            let mut coords = Vec::new();

            visited[i][j] = true;
            word.push_str(&board[i][j..=j]);
            coords.push((i as u8, j as u8));

            search_word(
                &board,
                &mut visited,
                &mut word,
                &mut coords,
                i,
                j,
                &mut found,
                &mut trie,
            );
        }
    }

    found
}

fn search_word(
    board: &[&str],
    visited: &mut Vec<Vec<bool>>,
    word: &mut String,
    coords: &mut Vec<(u8, u8)>,
    x: usize,
    y: usize,
    found: &mut HashMap<String, Vec<(u8, u8)>>,
    trie: &mut Trie,
) {
    let dirs = [
        (-1i8, -1i8),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    if trie.is_prefix(word) {
        let mut _valid_word = false; // Unused variable prefixed with underscore

        if trie.contains(word) {
            _valid_word = true; // Unused assignment prefixed with underscore
            found.insert(word.clone(), coords.clone());
        }

        for &(dx, dy) in &dirs {
            let new_x = x as i8 + dx;
            let new_y = y as i8 + dy;

            if new_x >= 0
                && new_x < board.len() as i8
                && new_y >= 0
                && new_y < board[0].len() as i8
                && !visited[new_x as usize][new_y as usize]
            {
                visited[new_x as usize][new_y as usize] = true;
                word.push_str(&board[new_x as usize][new_y as usize..=new_y as usize]);
                coords.push((new_x as u8, new_y as u8));
                search_word(
                    board,
                    visited,
                    word,
                    coords,
                    new_x as usize,
                    new_y as usize,
                    found,
                    trie,
                );
                word.pop();
                coords.pop();
                visited[new_x as usize][new_y as usize] = false;
            }
        }
    }
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl Trie {
    fn new(words: &[String]) -> Self {
        let mut trie = Trie::default();
        for word in words {
            trie.insert(word);
        }
        trie
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(TrieNode::default());
        }
        current.is_end = true;
    }

    fn contains(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            }
        }
        current.is_end
    }

    fn is_prefix(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
