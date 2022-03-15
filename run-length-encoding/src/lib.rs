pub fn encode(source: &str) -> String {
    let mut iter = source.as_bytes().iter();
    let first = if let Some(c) = iter.next() {
        c
    } else {
        return String::new();
    };
    let buff = String::with_capacity(source.len() / 2);

    let (count, last, mut buff) =
        source
            .as_bytes()
            .iter()
            .fold((0, first, buff), |(count, last, mut acc), c| {
                let count = if last == c {
                    count + 1
                } else {
                    if count > 1 {
                        acc.push_str(&count.to_string());
                    }
                    acc.push(*last as char);
                    1
                };
                (count, c, acc)
            });
    if count > 1 {
        buff.push_str(&count.to_string());
    }
    buff.push(*last as char);
    buff
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::with_capacity(source.len() * 2);
    let mut num = [ 0u8; 6];
    let mut num_idx = 0;
    for c in source.chars() {
        match c {
            '0'..='9' => {
                assert!(num_idx < 6);
                num[num_idx] = c as u8;
                num_idx += 1; 
            },
            c => {
                let count = if num_idx > 0 {
                    u32::from_str_radix(std::str::from_utf8(&num[..num_idx]).unwrap(), 10).unwrap()
                } else {
                    1
                };
                num_idx = 0;
                for _ in 1..=count {
                    decoded.push(c);
                }
            }
        }
    }
    decoded
}
