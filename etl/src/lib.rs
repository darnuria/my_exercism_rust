use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut tree, (&score, chars)| {
        tree.extend(chars.iter().map(|c| (c.to_ascii_lowercase(), score)));
        tree
    })
}
