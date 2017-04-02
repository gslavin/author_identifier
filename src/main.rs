use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// key words map to a word set
type WordChain = HashMap<VecDeque<String>, HashSet<String>>;

fn main() {

    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
        //parse file file
    }

	let f = File::open("pg5200.txt").unwrap();
	let mut f = BufReader::new(f);

    let mut text = String::new();
    f.read_to_string(&mut text).expect("Error reading file");
    let mut text_iter = text.split_whitespace();
    let mut key_words: VecDeque<String> =
        text_iter.by_ref().take(3).map(|s| s.to_string()).collect();

    let mut word_chain: WordChain = HashMap::new();
    for word in text_iter {
        if let None =  word_chain.get_mut(&key_words) {
            let word_set = HashSet::new();
            word_chain.insert(key_words.clone(), word_set);
        }
        let word_set = word_chain.get_mut(&key_words).unwrap();
        word_set.insert(word.to_string());
        key_words.pop_front();    
        key_words.push_back(word.to_string());
    }

    println!("max: {}", word_chain.values().map(|x| x.len()).max().unwrap());

    for (key_words, word_set) in &word_chain {

        println!("{:?} -> {:?}", key_words, word_set);
    }
}
