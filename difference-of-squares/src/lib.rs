// Much slower but close to the definition.
pub fn square_of_sum_iterating(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

// Much faster factorised version.
pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares_iterating(n: u32) -> u32 {
    (1..=n).fold(0, |acc, n| n.pow(2) + acc)
}

// geometric:
// https://www.youtube.com/watch?v=aXbT37IlyZQ&feature=youtu.be
// algebraic:
// https://www.youtube.com/watch?v=OpA7oNmHobM
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
