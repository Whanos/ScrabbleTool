use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::trie_data::Trie;

pub fn generate_trie() -> Trie {
    let mut trie = Trie::new();
    if let Ok(lines) = read_lines("wordlist.txt") {
        for line in lines {
            if let Ok(word) = line {
                trie.insert(word);
            }
        }
    }

    trie
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}