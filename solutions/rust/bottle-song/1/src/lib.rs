pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result: Vec<String> = Vec::new();
    for n in 0..take_down {
        let uppercase_text = match start_bottles - n {
            10 => "Ten green bottles",
            9 => "Nine green bottles",
            8 => "Eight green bottles",
            7 => "Seven green bottles",
            6 => "Six green bottles",
            5 => "Five green bottles",
            4 => "Four green bottles",
            3 => "Three green bottles",
            2 => "Two green bottles",
            1 => "One green bottle",
            _ => unreachable!(),
        };
        let lowercase_text = match start_bottles - 1 - n {
            0 => "no green bottles",
            9 => "nine green bottles",
            8 => "eight green bottles",
            7 => "seven green bottles",
            6 => "six green bottles",
            5 => "five green bottles",
            4 => "four green bottles",
            3 => "three green bottles",
            2 => "two green bottles",
            1 => "one green bottle",
            _ => unreachable!(),
        };
        let song = song(uppercase_text, lowercase_text);
        result.push(song);
    };
    result.into_iter().collect::<String>().trim().to_string()
}

fn song(start: &str, end: &str) -> String {
    format!(
"{start} hanging on the wall,
{start} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {end} hanging on the wall.\n\n"
    )
}
