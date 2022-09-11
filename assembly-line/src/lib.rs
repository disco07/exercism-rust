// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_RATE: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0
    };
    (speed as f64) * BASE_RATE * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
