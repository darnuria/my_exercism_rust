pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::from(""); digits.len() + 1],
        _ => digits
            .as_bytes()
            .windows(len)
            .map(|bytes| unsafe { String::from_utf8_unchecked(bytes.to_vec()) })
            .collect(),
    }
}
