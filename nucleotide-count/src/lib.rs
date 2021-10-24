use std::collections::HashMap;


#[inline]
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;
    if ['A', 'T', 'C', 'G'].iter().all(|&n| n != nucleotide) {
        return Err(nucleotide);
    }

    for n in dna.chars() {
        match n {
            'A' | 'T' | 'C' | 'G' if n == nucleotide => count += 1,
            'A' | 'T' | 'C' | 'G' => continue,
            n => return Err(n),
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut acc = HashMap::from([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
    for n in ['A', 'T', 'C', 'G'] {
        let count = count(n, dna)?;
        *acc.entry(n).or_insert(count) = count;
    }
    Ok(acc)
}
