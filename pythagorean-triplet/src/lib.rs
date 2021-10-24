use std::{collections::HashSet, hash::Hash};

// Reworked without srqt inspired by:
// https://exercism.org/tracks/rust/exercises/pythagorean-triplet/solutions/Acaccia
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut acc = HashSet::new();
    for a in 1..sum / 3 {
        for b in (a + 1)..sum / 2 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                let mut t = [a, b, c];
                t.sort();
                acc.insert(t);
            }
        }
    }
    acc
}
