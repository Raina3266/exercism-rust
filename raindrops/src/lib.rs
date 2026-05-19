pub fn raindrops(n: u32) -> String {
    let mut result = Vec::new();
    if n.is_multiple_of(3) {
        result.push("Pling".to_string());
    }
    if n.is_multiple_of(5) {
        result.push("Plang".to_string());
    }
    if n.is_multiple_of(7) {
        result.push("Plong".to_string());
    }
    if !n.is_multiple_of(3) && !n.is_multiple_of(5) && !n.is_multiple_of(7) {
        result.push(n.to_string());
    }
    result.into_iter().collect()
}
