// filter from 1 to limit -1, any factor who is multiple then sum it up.
// Being factor means l % f == 0.
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|l| {
            factors
                .iter()
                .filter(|&&factors| factors > 0)
                .any(|f| l % f == 0)
        })
        .sum()
}
