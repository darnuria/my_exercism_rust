use std::fmt::Write;

pub fn raindrops(n: u32) -> String {
    let s: [u8; 3] = [
        if n % 3 == 0 { 2 } else { 0 },
        if n % 5 == 0 { 4 } else { 0 },
        if n % 7 == 0 { 8 } else { 0 },
    ];

    match s.iter().sum() {
        0 => return n.to_string(),
        _ => s
            .iter()
            .map(|e| match e {
                2 => "Pling",
                4 => "Plang",
                8 => "Plong",
                _ => "",
            })
            .collect(),
    }
}

// Fun rustc/llvm produce similar asm as
// raindrop.
pub fn raindrops_bitwise(n: u32) -> String {
    let s: u8 = if n % 3 == 0 { 1 } else { 0 }
        | if n % 5 == 0 { 2 } else { 0 }
        | if n % 7 == 0 { 4 } else { 0 };

    match s {
        0 => n.to_string(),
        o => {
            let mut st = String::with_capacity(16);
            let a = if o & 1 == 1 { "Pling" } else { "" };
            let b = if o & 2 == 2 { "Plang" } else { "" };
            let c = if o & 4 == 4 { "Plong" } else { "" };
            write!(&mut st, "{}{}{}", a, b, c).unwrap();
            st
        }
    }
}
