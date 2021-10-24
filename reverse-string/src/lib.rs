use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Improved my solution based on PaulDance careful mentoring
    input.graphemes(true).rev().collect::<String>()
}
