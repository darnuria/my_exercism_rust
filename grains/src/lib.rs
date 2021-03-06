
// Compilation generated solution with a build script.
// Provides const GRAINS: [64; u64];
include!(concat!(env!("OUT_DIR"), "/grains.rs"));

// Hard coding of s = 54 solution to avoid using u128 numbers.
pub fn square_old(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        64 => 9_223_372_036_854_775_808u64,
        n if n > 64 => panic!("Square must be between 1 and 64"),
        _ => (2u64.pow(s) / 2),
    }
}

#[inline]
// Hardcoded solution with maybe totally a not idiomatic way XD
// But I did'nt wanted to use lazy-static.
pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        n if n > 64 => panic!("Square must be between 1 and 64"),
        n => GRAINS[(n -1) as usize]
       }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, n| acc + square(n))
}
