// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut count = HashMap::with_capacity(26);
    for word in magazine.iter() {
        for letter in word.chars() {
            println!("{}", letter);
            let mut count = count.entry(letter).or_insert(0);
            *count += 1;
        }
    }
    for word in note.iter() {
        for letter in word.chars() {
            match count.get_mut(&letter) {
                Some(mut count) if *count > 0 => {
                    println!("{}", count);
                    *count -= 1
                }
                _ => return false,
            }
        }
    }

    true
}
