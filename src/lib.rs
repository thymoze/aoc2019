use std::fs;

pub fn input(day: u8) -> String {
    fs::read_to_string(format!("../input/day{}", day)).unwrap()
} 