use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    for factor in factors {
        if factor != &0 {
            let mut count = 1;
            while count * factor < limit {
                let multiple = count * factor;
                if !set.contains(&multiple) {
                    set.insert(multiple);
                }
                count += 1;
            }
        }
    }
    set.iter().sum()
}
