// write the better solution

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

pub fn reply_better(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let mut has_letters = false;
    let mut all_uppercase = true;

    for c in trimmed.chars() {
        if c.is_ascii_alphabetic() {
            has_letters = true;

            if !c.is_uppercase() {
                all_uppercase = false;
            }
        }
    }

    let is_question = trimmed.ends_with('?');
    let is_yelling = has_letters && all_uppercase;

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever.",
    }
}
