/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // is there a size limit?
    let mut luhn = vec![];
    for x in code.chars() {
        if x.is_ascii_digit() {
            let x = x.to_digit(10).unwrap();
            luhn.push(x);
        } else if x == ' ' {
            continue;
        } else {
            return false;
        }
    }

    if luhn.len() < 2 {
        return false;
    }

    let parity = (luhn.len() - 2) % 2;
    let sum = luhn.iter()
        .enumerate()
        .rev()
        .map(|(i, &n)| {
            if i % 2 == parity {
                let doubled = n * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                n
            }
        })
        .sum::<u32>();
        sum % 10 == 0
}
