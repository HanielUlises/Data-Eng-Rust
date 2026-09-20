use sha3_dupe_detector::generate_random_phrases;

fn main() {
    let phrases = generate_random_phrases();
    sha3_dupe_detector::analyze_duplicates(&phrases);
}