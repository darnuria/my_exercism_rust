pub fn factors(mut n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut div = 2;
    while n >= (div * div) {
        println!("n: {} div: {}", n, div);
        if n % div == 0 {
            v.push(div);
            n = n / div;
        } else {
            div = div + 1;
        }
    }
    if n != 1 { v.push(n);}
    v
}
