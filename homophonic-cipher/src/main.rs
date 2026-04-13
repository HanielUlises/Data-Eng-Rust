use rand::prelude::*;
use std::collections::HashMap;

fn homophonic_cipher(plaintext: &str) -> (String, HashMap<char, Vec<char>>) {
    let mut rng = rand::rng();
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut ciphertext = String::new();
    let mut mapping: HashMap<char, Vec<char>> = HashMap::new();

    // Generate homophonic mapping
    for &c in &alphabet {
        let homophones: Vec<char> = (0..rng.random_range(2..4))
            .map(|_| rng.random_range('a'..='z'))
            .collect();
        mapping.insert(c, homophones);
    }

    // Encrypt plaintext
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let lower = c.to_ascii_lowercase();
            if let Some(homophones) = mapping.get(&lower) {
                let idx = rng.random_range(0..homophones.len());
                ciphertext.push(homophones[idx]);
            }
        } else {
            ciphertext.push(c);
        }
    }

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Mapping: {:?}", mapping);

    (ciphertext, mapping)
}

fn main() {
    let text = "Hello, World!";
    let (_cipher, _map) = homophonic_cipher(text);
}