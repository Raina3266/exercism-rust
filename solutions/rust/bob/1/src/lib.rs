pub fn reply(message: &str) -> &str {
    if message.is_empty() || message.chars().all(|char| char.is_whitespace()) {
        "Fine. Be that way!"
    } else if message
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .count()
        != 0
        && message
            .chars()
            .filter(|char| char.is_ascii_alphabetic())
            .all(|char| char.is_uppercase())
        && message.ends_with("?")
    {
        "Calm down, I know what I'm doing!"
    } else if message
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .count()
        != 0
        && message
            .chars()
            .filter(|char| char.is_ascii_alphabetic())
            .all(|char| char.is_uppercase())
    {
        "Whoa, chill out!"
    } else if message.trim().ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
