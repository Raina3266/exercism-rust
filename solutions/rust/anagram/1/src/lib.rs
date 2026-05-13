use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&'a str> = HashSet::new();
    let mut target: HashMap<char, u32> = HashMap::new();
    let word_lower = word.to_lowercase();
    for char in word_lower.chars() {
        *target.entry(char).or_insert(0) += 1;
    }
    for possible_word in possible_anagrams {
        let lower_word = possible_word.to_lowercase();
        let mut word_set: HashMap<char, u32> = HashMap::new();
        for char in lower_word.chars() {
            *word_set.entry(char).or_insert(0) += 1;
        }
        if word_set == target && word_lower != lower_word {
            set.insert(possible_word);
        }
    }
    set
}
