use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::cmp;

// key words map to a map of word counts
type WordMap = HashMap<VecDeque<String>, HashMap<String, u64>>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct WordChain {
    key_length: usize,
    word_map: WordMap
}

impl fmt::Display for WordChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key_words, word_counts) in &self.word_map {
            if let Err(x) = write!(f, "{:?} -> {:?})\n", key_words, word_counts) {
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
            let mut word_count = word_map.entry(key_words.clone()).or_insert(HashMap::new());
            (*(*word_count).entry(word.to_string()).or_insert(0)) += 1;
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
            let word_counts: &HashMap<String, u64> = self.word_map.get(&key).unwrap();
            let word_counts_other: &HashMap<String, u64> = other.word_map.get(&key).unwrap();
            let words: HashSet<_> = word_counts.keys().collect();
            let words_other: HashSet<_> = word_counts_other.keys().collect();
            let mut intersection_size = 0;

            for word in words.intersection(&words_other) {
               intersection_size += *word_counts.get(word as &str).unwrap();
               intersection_size += *word_counts_other.get(word as &str).unwrap();
            }

            let max_size: u64 = cmp::max(word_counts.values().sum(), word_counts_other.values().sum());

            // Create a normailized sum so identical texts have a similarity of 1
            result += (intersection_size as f64) / ((2 * max_size) as f64);
        }

        // Create a normailized sum so identical texts have a similarity of 1
        result = result / (intersection.len() as f64);

        return Some(result);
    }

    fn merge(&self, other: &WordChain) -> WordChain {
        // creates a merged map from the two given references
        let mut new_word_map = HashMap::new();
        let word_maps = vec![&self.word_map, &other.word_map];

        for word_map in word_maps {
            for (key, word_counts) in word_map.clone() {
                let mut new_word_count =
                    new_word_map.entry(key).or_insert(HashMap::new());
                for word in word_counts.keys() {
                   (*new_word_count.entry(word.clone()).or_insert(0)) +=
                       *word_counts.get(word).unwrap();
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

    let key_words: VecDeque<String> = vec![String::from("This"), String::from("is")].into_iter().collect();

    let expected_word_counts: HashMap<String, u64> = vec![(String::from("a"), 1)].into_iter().collect();
    let word_counts = word_chain.word_map.get(&key_words).unwrap();
    assert_eq!(expected_word_counts, *word_counts);
}

#[test]
fn word_chain_new_repeat_words() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a This is a");

    let word_chain = WordChain::new(text, KEY_LENGTH);

    let key_words: VecDeque<String> = vec![String::from("This"), String::from("is")].into_iter().collect();

    let expected_word_counts: HashMap<String, u64> = vec![(String::from("a"), 2)].into_iter().collect();
    let word_counts = word_chain.word_map.get(&key_words).unwrap();
    assert_eq!(expected_word_counts, *word_counts);
}

#[test]
fn word_chain_compare_eq() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");

    let word_chain = WordChain::new(text, KEY_LENGTH);


    assert_eq!(word_chain.compare(&word_chain.clone()).unwrap(), 1.0);
}

#[test]
fn word_chain_compare_ne() {
    const KEY_LENGTH: usize = 2;
    let text = String::from("This is a test");
    let text_2 = String::from("No common words in text");

    let word_chain = WordChain::new(text, KEY_LENGTH);
    let word_chain_2 = WordChain::new(text_2, KEY_LENGTH);

    assert_eq!(word_chain.compare(&word_chain_2).unwrap(), 0.0);
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

    let expected_word_counts: HashMap<String, u64> =
        vec![(String::from("a"), 1), (String::from("the"), 1)].into_iter().collect();
    let word_counts = new_word_chain.word_map.get(&key_words).unwrap();
    assert_eq!(expected_word_counts, *word_counts);
}
