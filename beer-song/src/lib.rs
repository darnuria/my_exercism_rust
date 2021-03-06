use std::borrow::Cow;

pub fn verse(n: u32) -> String {
    let (quantity, modifier, left) = match n {
        0 => ("no more".into(), "s", "99 bottles".into()),
        1 => ("1".into(), "", "no more bottles".into()),
        2 => ("2".into(), "s", "1 bottle".into()),
        n => (
            Cow::Owned(n.to_string()),
            "s",
            Cow::Owned(format!("{} bottles", (n - 1).to_string())),
        ),
    };

    let middle = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };

    // ¯\_(ツ)_/¯ Not really nice but it's OKAY.
    let mut s = format!(
        "{} bottle{} of beer on the wall, {} bottle{} of beer.\n{}, {} of beer on the wall.\n",
        quantity, modifier, quantity, modifier, middle, left
    );
    if let Some(c) = s.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    s
}

pub fn sing(start: u32, end: u32) -> String {
    // Inspired by https://exercism.io/tracks/rust/exercises/beer-song/solutions/64590f95e84c460484034eb7cc2bfc29
    // changed the collect() type to Vec<_> thanks: peakmorgana's
    (end..=start)
        .rev()
        .map(|i| verse(i))
        .collect::<Vec<_>>()
        .join("\n")
}
