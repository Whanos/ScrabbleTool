use std::time::{Instant};
use crate::trie_data::Trie;
use crate::trie_generate::generate_trie;

mod trie_data;
mod trie_generate;
mod combinations;
mod scorer;

fn main() {
    println!("Scrabble Solver");
    let start = Instant::now();
    let trie = generate_trie();
    let duration = start.elapsed();
    println!("Generated Trie. Time taken: {:?}", duration);

    let words = generate_words_from_hand("JILURSV".to_string(), trie);
    for word in words {
        println!{"{} - {}", word, scorer::score_word(word.clone())};
    }
}

fn generate_words_from_hand(hand: String, trie: Trie) -> Vec<String> {
    let mut words = Vec::new();

    for set in combinations::power_set(hand.as_str()) {
        for word in combinations::permutations(&set) {
            if trie.search(word.clone()) {
                words.push(word);
            }
        }
    }

    words
}