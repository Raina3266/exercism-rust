pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut prime = 2;
    let mut num = n;
    while num > 1 {
        if is_prime(prime) && num.is_multiple_of(prime) {
            num /= prime;
            factors.push(prime);
        } else {
            prime += 1;
        }
    }
    factors
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }

    if num.is_multiple_of(2) || num.is_multiple_of(3) {
        return false;
    }

    let mut k = 5;

    while k * k <= num {
        if num.is_multiple_of(k) || num.is_multiple_of(k + 2) {
            return false;
        }
        k += 6;
    }
    true
}
