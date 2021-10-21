use std::cmp::Ordering;

pub fn find(mut array: &[i32], key: i32) -> Option<usize> {
    let kay = Some(&key);
    if array.first() > kay || array.last() < kay {
        return None;
    }
    let mut offset = 0;
    loop {
        if array.is_empty() {
            break None;
        }
        let mid = array.len() / 2;
        match key.cmp(&array[mid]) {
            Ordering::Greater => {
                let new_start = array.len() - mid;
                array = &array[new_start..];
                offset += new_start;
            }
            Ordering::Equal => return Some(offset + mid),
            Ordering::Less => array = &array[..mid],
        }
    }
}
