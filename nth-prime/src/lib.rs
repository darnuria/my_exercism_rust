// algorithm from wikipedia: https://en.wikipedia.org/wiki/Primality_test
// It's kinda raw but... fast? O.o'
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
    (2..)
        .filter(|&e| is_prime(e))
        .nth((n + 1) as usize)
        .unwrap()
}
