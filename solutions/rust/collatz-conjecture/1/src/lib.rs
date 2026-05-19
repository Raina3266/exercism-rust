use std::collections::HashSet;

pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut set: HashSet<u64> = HashSet::new();
    let mut count = 0;
    while n != 1 {
        if n.is_multiple_of(2) {
            n /= 2;
            count += 1;
            if set.contains(&n) {
                return None;
            } else {
                set.insert(n);
            }
        } else {
            n = n * 3 + 1;
            count += 1;
            if set.contains(&n) {
                return None;
            } else {
                set.insert(n);
            }
        }
    }
    Some(count)
}

