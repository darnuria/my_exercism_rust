
pub fn square_of_sum_v0(n: u32) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1; // Can I haz I++? Nay.
    }
    sum * sum
}

pub fn square_of_sum_v1(n: u32) -> u32 {
    // applique F x, acc -> x + acc, tel que acc = 0 pour 1 à N inclus
    (1..=n).fold(0, |acc, x| acc + x).pow(2)
}

pub fn square_of_sum_v2(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares_v0(n: u32) -> u32 {
    let mut range = 1..=n;
    let mut sum = 0;
    while let Some(x) = range.next() {
        sum += x * x;
    }
    sum
}

pub fn sum_of_squares_v1(n: u32) -> u32 {
    let mut sum = 0;
    for x in 1..=n {
        sum += x * x;
    }
    sum
}

pub fn sum_of_squares_v2(n: u32) -> u32 {
    (1..=n).fold(0, |acc, x| acc + x * x)
}

// geometric:
// https://www.youtube.com/watch?v=aXbT37IlyZQ
// algebraic:
// https://www.youtube.com/watch?v=OpA7oNmHobM
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
