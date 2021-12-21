use std::cmp::Ordering;

// Thanks a mentoring I learned about ? for Option. :)
pub fn find(mut array: &[i32], key: i32) -> Option<usize> {
    if array.first()? > &key || array.last()? < &key {
        return None;
    }
    let mut offset = 0;
    while !array.is_empty() {
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
    None
}
