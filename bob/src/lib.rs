pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    // also possible to do |c| ("A"..="Z").contains(&c)

    // inspired by https://exercism.io/tracks/rust/exercises/bob/solutions/abf2117b2c27413186b5e847f29cf8fe
    // legeana's solution
    let has_alpha = trimmed.chars().any(|c| c.is_alphabetic());
    let yelling = has_alpha && trimmed.chars().all(|c| !c.is_alphabetic() || c.is_ascii_uppercase());
    // let numbers_of_caps = trimmed.matches(|c| c.is_ascii_uppercase()).count();
    // let numbers_of_letters = trimmed.matches(|c| c.is_ascii_lowercase()).count();
    // let yelling = numbers_of_caps > numbers_of_letters;
    if trimmed.len() == 0 {
        return "Fine. Be that way!";
    }

    if trimmed.ends_with("?") {
        if !yelling {
            return "Sure.";
        }
        return "Calm down, I know what I'm doing!";
    }

    if yelling {
        return "Whoa, chill out!"
    }
    "Whatever."
}
