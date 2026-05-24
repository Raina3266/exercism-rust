pub fn abbreviate(phrase: &str) -> String {
    let mut result: Vec<char> = vec![];
    for part in phrase.split(&[' ', '-', '_']) {
        let chars: Vec<char> = part.chars().collect();
        if chars.is_empty() {
            continue;
        } else {
            result.push(chars[0].to_ascii_uppercase());
            for i in 1..chars.len() {
                if chars[i].is_ascii_uppercase() && chars[i - 1].is_ascii_lowercase() {
                    result.push(chars[i]);
                }
            }
        }
    }
    result.iter().collect::<String>()
}
