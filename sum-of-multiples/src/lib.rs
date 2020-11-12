pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = std::collections::HashSet::new();
    let mut s = 0;
    for x in 0..limit {
        for &y in factors {
            if y == 0 {
                continue;
            }
            if x % y == 0 && !multiples.contains(&x) {
                s += x;
                multiples.insert(x);
            }
        }
    }
    s
}
