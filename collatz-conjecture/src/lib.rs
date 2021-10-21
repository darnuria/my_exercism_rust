pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    for i in 0.. {
        n = match n {
            1 => return Some(i),
            n if n % 2 != 0 => n.checked_mul(3)?.checked_add(1)?,
            n => n / 2,
        };
    }
    None
}
