// SRC0: https://seenaburns.com/benchmarking-rust-with-cargo-bench
// src1: https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html

#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    
    #[bench]
    fn iter_0_1000000(b: &mut Bencher) {
        b.iter(|| {
            for y in 0..1000000 {
                black_box(leap::is_leap_year(y));
            }
        });
    }

    #[bench]
    fn iter_1_1000000(b: &mut Bencher) {
        b.iter(|| {
            for y in 0..1000000 {
                black_box(leap::is_leap_year_overcharged(y));
            }
        });
    }

    #[bench]
    fn iter_2_1000000(b: &mut Bencher) {
        b.iter(|| {
            for y in 0..1000000 {
                black_box(leap::is_leap_year_if(y));
            }
        });
    }

    #[bench]
    fn iter_3_1000000(b: &mut Bencher) {
        b.iter(|| {
            for y in 0..1000000 {
                black_box(leap::is_leap_year_if_not(y));
            }
        });
    }

    #[bench]
    fn iter_4_1000000(b: &mut Bencher) {
        b.iter(|| {
            for y in 0..1000000 {
                black_box(leap::is_leap_year_group(y));
            }
        });
    }
}
