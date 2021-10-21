pub fn brackets_are_balanced_stack(string: &str) -> bool {
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
use core::str::Chars;

enum Delim {Curly, Squared, Parens}
fn inner(c: Option<char>, mut s : Chars) -> Option<char> {
    match (c, s.next()) {
        (Some(d) , c @ (Some('}') | Some(']') | Some(')'))) => if Some(c) != Some(c) { return Some('@') } else { inner(None, s) },
        (_ ,Some('{')) => inner(Some('}'), s) ,
        (_ ,Some('[')) => inner(Some(']'), s) ,
        (_ ,Some(')')) => inner(Some(')'), s) ,
        (Some(c), None) => Some(c),
        (None, None) => None,
        (None, Some(_)) => inner(None, s),
        (Some(_), Some(_)) => inner(None, s)
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {

    let rest = inner(None, string.chars());
    rest.is_none()
}
