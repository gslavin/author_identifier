use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::cmp;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// key words map to a word set
type WordMap = HashMap<VecDeque<String>, HashSet<String>>;
struct WordChain {
    key_length: usize,
    word_map: WordMap
}

impl WordChain {
    // convert a block of text into a frequency histogram
    fn new(text: String, key_length: usize) -> WordChain {
        let mut text_iter = text.split_whitespace();
        let mut key_words: VecDeque<String> =
            text_iter.by_ref().take(key_length).map(|s| s.to_string()).collect();

        let mut word_map: WordMap = HashMap::new();
        for word in text_iter {
            if let None =  word_map.get_mut(&key_words) {
                let word_set = HashSet::new();
                word_map.insert(key_words.clone(), word_set);
            }
            let word_set = word_map.get_mut(&key_words).unwrap();
            word_set.insert(word.to_string());
            key_words.pop_front();
            key_words.push_back(word.to_string());
        }

        return WordChain{ key_length: key_length, word_map: word_map};
    }

    // Compare word chains
    // Gives a score between 0 and 1 that shows how similar the texts are
    // assumes a is the larger word chain
    fn compare(&self, other: &WordChain) -> Option<f64> {
        if self.key_length != other.key_length {
            return None;
        }
        // find insersection of keys
        let mut result = 0.0;
        let keys: HashSet<_> = self.word_map.keys().cloned().collect();
        let keys_other: HashSet<_> = other.word_map.keys().cloned().collect();

        // for intersection of keys compare sets
        let intersection: HashSet<_> =
            keys.intersection(&keys_other).cloned().collect();
        for key in &intersection {
            let values: &HashSet<_> = self.word_map.get(&key).unwrap();
            let values_other: &HashSet<_> = other.word_map.get(&key).unwrap();
            let intersection_size =
                (*values).intersection(values_other).count() as f64;
            let max_size = cmp::max(values.len(), values_other.len());
            result += intersection_size / (max_size as f64);
        }

        result = result / (intersection.len() as f64);
        if result.is_nan() {
            result = 0.0;
        }

        return Some(result);
    }
}

fn main() {
    let (size, _) = env::args().size_hint();
    if size < 3 {
        println!("author_identifier [text] [comparison text]");
        return;
    }
    const KEY_LENGTH: usize = 2;

    let mut word_chains: Vec<WordChain> = Vec::new();

    for path in env::args().skip(1) {
        let f = File::open(path).unwrap();
        let mut f = BufReader::new(f);
        let mut text = String::new();
        f.read_to_string(&mut text).expect("Error reading file");
        let word_chain = WordChain::new(text, KEY_LENGTH);
        word_chains.push(word_chain);
    }

    /*
    println!("max: {}",
        word_chain.values().map(|x| x.len()).max().unwrap());

    for (key_words, word_set) in &word_chain {
        println!("{:?} -> {:?}", key_words, word_set);
    }
    */

    let sim = word_chains[0].compare(&word_chains[1]).expect("Unable to compare word chains");
    println!("{}", sim);
}
