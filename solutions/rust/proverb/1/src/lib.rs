pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = Vec::new();
    if list.len() == 1 {
        proverb.push(format!("And all for the want of a {}.", list[0]));
    }
    if list.len() > 1 {
        for n in 1..list.len() {
            proverb.push(format!(
                "For want of a {a} the {b} was lost.\n",
                a = list[n - 1],
                b = list[n]
            ));
        }
        proverb.push(format!("And all for the want of a {}.", list[0]));
    }
    proverb.into_iter().collect()
}
