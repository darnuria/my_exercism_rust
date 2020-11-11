use chrono::{DateTime, Utc};

/// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // I know I can global import but I like to know from where things comes.
    let giga_sec = chrono::Duration::seconds(1_000_000_000);
    start + giga_sec
}
