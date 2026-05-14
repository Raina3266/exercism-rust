/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned: Vec<char> = code.chars().filter(|char| !char.is_whitespace()).collect();
    if cleaned.len() <= 1 || cleaned.iter().any(|char| !char.is_ascii_digit()) {
        return false;
    }
    cleaned
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, char)| {
            if index % 2 == 1 {
                let result = char.to_digit(10).unwrap() * 2;
                if result > 9 {
                    return result - 9;
                }
                return result;
            }
            char.to_digit(10).unwrap()
        })
        .sum::<u32>()
        % 10
        == 0
}
