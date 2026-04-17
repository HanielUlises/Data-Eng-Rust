use std::{collections::HashMap, hash::Hash, os::linux::raw::stat};

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
    let freq_eng_map: HashMap<char, f32> =     eng_freq_map.iter().map(|(k, v)| (*k, *v)).collect();

    let mut results = Vec::new();

    for(letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let eng_freq = eng_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let eng_freq_diff = eng_freq.map_or(0.0, |f|(freq - f).abs());

        results.push((*letter, *count, freq, eng_freq, eng_freq_diff));
    }  

    results
} 

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    for(letter, count, freq, eng_freq, eng_freq_diff) in stats {
        println!("{}: {} ({}%), English Freq: {} ({}%)", 
                letter, count, freq, eng_freq.unwrap_or(0.0), eng_freq_diff)
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a'} else { b'A'};
            let offset = (c as u8 - base + shit) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}