pub fn nth(n: u32) -> u32 {
    if n > 1000 {
        return nth_with_caching(n)
    } else {
        return nth_naive(n)
    }
}

pub fn nth_naive(n: u32) -> u32 {
    let mut primes_found: u32 = 0;

    for candidate in 2.. {
        if is_prime(candidate) {
            primes_found += 1;
            if primes_found == n + 1 {
                return candidate
            }
        }
    }

    return 0;
}

pub fn is_prime(n: u32) -> bool {
    for m in 2..=n/2 {
        if n % m == 0 {
            return false
        }
    }

    return true;
}

// nth_with_caching determines the nth prime.
// will use more memory than nth_naive but will be faster for large n.
pub fn nth_with_caching(n: u32) -> u32 {
    let mut primes_found: u32 = 0;
    let mut primes: Vec<u32> = Vec::with_capacity(n as usize);

    for candidate in 2.. {
        if is_prime_plus(candidate, &primes) {
            if primes_found == n {
                return candidate
            }
            primes.push(candidate);
            primes_found += 1;
        }
    }

    return 0;
}

// is_prime_plus returns true if n is prime.
// if not empty, first_x_primes must be the set of the first x primes.
pub fn is_prime_plus(n: u32, first_x_primes: &Vec<u32>) -> bool {
    for prime in first_x_primes {
        if *prime > n/2 {
            return true
        } else if n % prime == 0 {
            return false
        }
    }

    let start_at = match first_x_primes.last() {
        None => 2,
        Some(x) => x + 1
    };

    for m in start_at..=n/2 {
        if n % m == 0 {
            return false
        }
    }

    return true
}
