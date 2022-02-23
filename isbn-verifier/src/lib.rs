/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut iter = isbn.bytes().rev();
    let check = if let Some(c) = iter
        .next()
        .map(|c| match c {
            b'X' | b'x' => Some(10),
            _ => (c as char).to_digit(10),
        })
        .flatten()
    {
        c
    } else {
        return false;
    };

    let (counter, sum) = iter
        .filter_map(|c| (c as char).to_digit(10))
        .fold((2, 0), |(count, sum), x| (count + 1, sum + x * count));
    (sum + check) % 11 == 0 && counter == 11
}
