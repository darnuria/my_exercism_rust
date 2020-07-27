// algorithm wikipedia: https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false
    };

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false
        }
        i = i + 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i = 2;
    while primes.len() <= n as usize {
        if is_prime(i) { 
            primes.push(i);
        }
        i += 1;
    }
    primes[n as usize]
}
