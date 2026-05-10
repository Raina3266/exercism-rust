pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_order = match student {
        "Alice" => 1,
        "Bob" => 2,
        "Charlie" => 3,
        "David" => 4,
        "Eve" => 5,
        "Fred" => 6,
        "Ginny" => 7,
        "Harriet" => 8,
        "Ileana" => 9,
        "Joseph" => 10,
        "Kincaid" => 11,
        "Larry" => 12,
        _ => unreachable!(),
    };
    let length = diagram.len();
    diagram
        .chars()
        .enumerate()
        .filter(|(index, _)| {
            *index == student_order * 2 - 1
                || *index == student_order * 2 - 2
                || *index == student_order * 2 - 1 + length / 2
                || *index == student_order * 2 + length / 2
        })
        .map(|(_, char)| match char {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unreachable!(),
        })
        .collect()
}
