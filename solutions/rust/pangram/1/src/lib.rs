use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut map: HashMap<char, usize> = HashMap::new();
    for char in alphabet {
        map.entry(char).or_insert(0);
    }
    for char in sentence.chars().map(|char| char.to_ascii_lowercase()) {
        if map.contains_key(&char) {
            *map.get_mut(&char).unwrap() += 1;
        }
    }
    map.into_values().all(|count| count >= 1)
}
