// algorithm from wikipedia: https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    // Unwraped because the exercism signature do not let
    // expression of failure. :(
    (2..)
        .filter(|&e| is_prime(e))
        .take((n + 1) as usize)
        .last()
        .unwrap()
}
