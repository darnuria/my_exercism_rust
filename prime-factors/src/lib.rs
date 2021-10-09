// second iteration largely based on this blog article
// https://programmingpraxis.com/2009/05/08/wheel-factorization/
// And solution purposed on stackoverflow:
// https://stackoverflow.com/questions/12756335/fast-prime-factorization-algorithm/12759741#12759741
pub fn factors(mut n: u64) -> Vec<u64> {
    let wheel = [1, 2, 2, 4, 2, 4, 2, 4, 6, 2, 6];
    let mut factor = 2;
    let mut i = 0;
    let mut v = Vec::new();
    while n >= factor * factor {
        if n % factor == 0 {
            v.push(factor);
            n /= factor;
        } else {
            factor += wheel[i];
            i = if i == 10 { 3 } else { i + 1 };
        }
    }
    if n != 1 {
        v.push(n);
    }
    v
}
