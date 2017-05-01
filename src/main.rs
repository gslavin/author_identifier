use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::cmp;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

// key words map to a word set
type WordMap = HashMap<VecDeque<String>, HashSet<String>>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct WordChain {
    key_length: usize,
    word_map: WordMap
}

impl fmt::Display for WordChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key_words, word_set) in &self.word_map {
            if let Err(x) = write!(f, "{:?} -> {:?})\n", key_words, word_set) {
                return Err(x);
            }
        }
        return Ok(());
    }
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

        if intersection.is_empty() {
            return Some(0.0);
        }

        for key in &intersection {
            let values: &HashSet<_> = self.word_map.get(&key).unwrap();
            let values_other: &HashSet<_> = other.word_map.get(&key).unwrap();
            let intersection_size =
                (*values).intersection(values_other).count() as f64;
            let max_size = cmp::max(values.len(), values_other.len());
            result += intersection_size / (max_size as f64);
        }

        result = result / (intersection.len() as f64);

        return Some(result);
    }

    fn merge(&self, other: &WordChain) -> WordChain {
        // creates a merged map from the two given references
        let mut new_word_map = HashMap::new();
        let word_maps = vec![&self.word_map, &other.word_map];

        for word_map in word_maps {
            for (key, value_set) in word_map.clone() {
                let mut entry = new_word_map.entry(key).or_insert(HashSet::new());
                for v in value_set.clone().into_iter() {
                    (*entry).insert(v);
                }
            }
        }

        return WordChain{ key_length: self.key_length, word_map: new_word_map};
    }

    fn len(&self) -> usize {
        return self.word_map.len();
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

    //println!("{}", word_chains[0]);
    let sim = word_chains[0].compare(&word_chains[1]).expect("Unable to compare word chains");
    println!("{}", sim);
}


#[test]
fn word_chain_new() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");

    let word_chain = WordChain::new(text, KEY_LENGTH);

    assert_eq!(word_chain.len(), 2);
}

#[test]
fn word_chain_compare() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");
    let text_2 = String::from("No common words in text");

    let word_chain = WordChain::new(text, KEY_LENGTH);
    let word_chain_2 = WordChain::new(text_2, KEY_LENGTH);

    assert_ne!(word_chain, word_chain_2);
}

#[test]
fn word_chain_merge() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");
    let text_2 = String::from("No common words in text");

    let word_chain = WordChain::new(text, KEY_LENGTH);
    let word_chain_2 = WordChain::new(text_2, KEY_LENGTH);
    let new_word_chain = word_chain.merge(&word_chain_2);

    assert_eq!(new_word_chain.len(), word_chain.len() + word_chain_2.len());
}

#[test]
fn word_chain_merge_common() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");
    let text_2 = String::from("This is the test");

    let word_chain = WordChain::new(text, KEY_LENGTH);
    let word_chain_2 = WordChain::new(text_2, KEY_LENGTH);
    let new_word_chain = word_chain.merge(&word_chain_2);

    let key_words: VecDeque<String> = vec![String::from("This"), String::from("is")].into_iter().collect();

    let mut expected_word_set = HashSet::new();
    expected_word_set.insert(String::from("a"));
    expected_word_set.insert(String::from("the"));
    let word_set = new_word_chain.word_map.get(&key_words).unwrap();
    assert_eq!(expected_word_set, *word_set);
}
