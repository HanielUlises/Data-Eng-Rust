use std::{collections::HashMap, hash::Hash};

fn gen_counts() -> HashMap<char, f32> {
    let mut eng_freq: HashMap<char, f32> = HashMap::new();

    eng_freq.insert('e', 13.7);
    eng_freq.insert('t', 9.1);
    eng_freq.insert('a', 8.2);
    eng_freq.insert('o', 7.5);
    eng_freq.insert('i', 7.0);
    eng_freq.insert('n', 6.7);
    eng_freq.insert('s', 6.3);
    eng_freq.insert('h', 6.1);
    eng_freq.insert('r', 6.0);
    eng_freq.insert('d', 4.3);

    eng_freq
}

fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();
    let eng_freq_map = gen_counts();
    let freq_eng_map: HashMap<char, f32> = eng_freq_map.iter().map(|k, v| (*k, *v))
} 