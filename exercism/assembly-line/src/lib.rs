// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let gross_production = (speed as f64) * 221f64;
    let success_rate: f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        _ => 0.77
    };
    gross_production * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32)/60
}
