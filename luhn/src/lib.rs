/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let iter = code.chars().rev().filter(|c| !c.is_whitespace());

    let mut i = 0;
    let mut sum = 0;
    for n in iter {
        let n = if let Some(n) = n.to_digit(10) {
            n
        } else {
            return false;
        };

        let n = if i % 2 == 1 {
            let doubled = n * 2;
            if doubled > 9 {
                doubled - 9
            } else {
                doubled
            }
        } else {
            n
        };
        i += 1;
        sum += n;
    }
    sum % 10 == 0 && i > 1
}
