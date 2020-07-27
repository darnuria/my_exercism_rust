#[inline]
fn count_digit(n: u32, base: u32) -> u32 {
    let mut count_digits = 0;
    let mut rest = n;
    while rest > 0 {
        rest /= base;
        count_digits += 1;
    }
    count_digits
}

// Solution derived from wikipedia article.
// https://en.wikipedia.org/wiki/Narcissistic_number
pub fn is_armstrong_number(num: u32) -> bool {
    let base = 10;
    let count_digits = count_digit(num, base);
    let mut current_digit = num;
    let mut sum = 0;
    while current_digit > 0 {
        sum += (current_digit % base).pow(count_digits);
        current_digit /= base;
    }
    sum == num
}
