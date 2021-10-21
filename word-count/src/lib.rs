use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|w: char| w.is_whitespace() || w == ',')
        .map(|w| w.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|s| !s.is_empty())
        .fold(HashMap::new(), |mut acc, w| {
            let w = w.to_ascii_lowercase();
            let entry = acc.entry(w).or_insert(0);
            *entry += 1;
            acc
        })
}
