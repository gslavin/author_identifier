use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// key words map to a word set
type WordChain = HashMap<VecDeque<String>, HashSet<String>>;

// convert a block of text into a frequency histogram
fn create_word_chain(text: String, key_length: usize) -> WordChain {
    let mut text_iter = text.split_whitespace();
    let mut key_words: VecDeque<String> =
        text_iter.by_ref().take(key_length).map(|s| s.to_string()).collect();

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

    return word_chain;
}

// compare word chains
// Gives a score between 0 and 1 that shows how similar the texts are
// assumes a is the larger word chain
fn compare_word_chains(a: &WordChain, b: &WordChain) -> f64 {
    // find insersection of keys
    let mut result = 0.0;
    let keys_a: HashSet<_> = a.keys().cloned().collect();
    let keys_b: HashSet<_> = b.keys().cloned().collect();

    // for intersection of keys compare sets
    let intersection: HashSet<_> =
        keys_a.intersection(&keys_b).cloned().collect();
    for key in &intersection {
        let values_a: &HashSet<_> = a.get(&key).unwrap();
        let values_b: &HashSet<_> = b.get(&key).unwrap();
        let intersection_size =
            (*values_a).intersection(values_b).count() as f64;
        result += intersection_size / (values_a.len() as f64);
    }

    result = result / (intersection.len() as f64);
    if result.is_nan() {
        result = 0.0;
    }

    return result;
}


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
    let word_chain = create_word_chain(text, 3);

	let f = File::open("snippet.txt").unwrap();
	let mut f = BufReader::new(f);
    let mut text_2 = String::new();
    f.read_to_string(&mut text_2).expect("Error reading file");
    let word_chain_2 = create_word_chain(text_2, 3);

    /*
    println!("max: {}",
        word_chain.values().map(|x| x.len()).max().unwrap());

    for (key_words, word_set) in &word_chain {
        println!("{:?} -> {:?}", key_words, word_set);
    }
    */

    let sim = compare_word_chains(&word_chain, &word_chain_2);
    println!("{}", sim);
}
