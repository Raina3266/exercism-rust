pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();

    for word in phrase.split(&[' ', '-', '_']) {
        let chars: Vec<char> = word.chars().collect();

        if chars.is_empty() {
            continue;
        }
        result.push(chars[0].to_ascii_uppercase());

        for i in 1..chars.len() {
            if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                result.push(chars[i]);
            }
        }
    }
    result
}