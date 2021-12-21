// Not too much original stack version
// Downside: allocate on heap
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut delimiters = Vec::new();
    for c in string.chars() {
        match c {
            '{' => delimiters.push('}'),
            '(' => delimiters.push(')'),
            '[' => delimiters.push(']'),
            ']' | '}' | ')' => {
                if delimiters.pop() != Some(c) {
                    return false;
                }
            }
            _ => continue,
        }
    }
    delimiters.len() == 0
}

// TODO: stack version
// Upside : dont allocate; may crash on big input if designed too naively.
