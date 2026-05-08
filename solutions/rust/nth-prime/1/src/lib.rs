pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    // 0 and 1 are not prime numbers
    if n <= 1 {
        return false;
    }
    // 2 and 3 are prime numbers
    if n <= 3 {
        return true;
    }

    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    // We step by 6 because all primes are of the form 6k ± 1.
    let mut i = 5;
    
    // Check odd numbers starting from 5 up to the square root of n.
    while i * i <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}
