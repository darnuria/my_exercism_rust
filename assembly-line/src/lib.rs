// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.


#![feature(exclusive_range_pattern)]

/// Items per hours of the assembly line
const ITEMS_PER_HOURS:f64 = 221.;

/// Caculate the production rate in the factory based on given speed.
///
/// `speed` modes documentation:
///
/// There is 4 speed level on the assembly line
/// speed is represented as an 8 bit unsigned integer `u8` designed as `speed`.
/// 
/// Speed modes implication:
///
/// 0 : Nothing goes out
/// 1 to 4 included: full production
/// 1 to 8 included: 90% percents of the items factored
/// 9 to 10 included: only 77% of the items are factored
///
/// Speed is a tradeof between lost rate and production rate.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let failure_rate = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => unimplemented!("not designed for!"),
    };
    failure_rate * ITEMS_PER_HOURS * speed as f64
}

/// Get working items produced per minte in assembly line.
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
