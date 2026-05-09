pub fn is_armstrong_number(num: u32) -> bool {
    let number_of_digit = if num == 0 { 1 } else { num.ilog10() + 1 };
}
