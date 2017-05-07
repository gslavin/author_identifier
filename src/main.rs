#![feature(alloc_system)]
extern crate alloc_system;

use std::env;

mod word_chain;
use word_chain::WordChain;

fn main() {
    let (size, _) = env::args().size_hint();
    if size < 3 {
        println!("author_identifier [text] [comparison text]");
        return;
    }
    const KEY_LENGTH: usize = 2;

    let mut text_paths = env::args().skip(1);

    let sample_path = text_paths.next().unwrap();
    let sample_word_chain = WordChain::from_path(sample_path.clone(), &sample_path, KEY_LENGTH);
    println!("Comparisons for {}", sample_word_chain.title);
    for path in text_paths {
        let word_chain = WordChain::from_path(path.clone(), &path, KEY_LENGTH);
        let sim = sample_word_chain.compare(&word_chain).expect("Unable to compare word chains");
        println!("{}: {}", word_chain.title, sim);
    }
}
