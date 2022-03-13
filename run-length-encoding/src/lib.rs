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
    unimplemented!("Return the run-length decoding of {}.", source);
}
