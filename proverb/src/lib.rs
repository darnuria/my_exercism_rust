// Update;
// Inspired by KirmesBude solution.
pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            let proverb = list
                .windows(2)
                .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
                .collect::<String>();
            format!("{}And all for the want of a {}.", proverb, list[0])
        }
    }
}
