mod checks;

use checks::{AdjacentGroup, AdjacentPair, AlwaysIncreases};
use common::input;

fn main() {
    let input = input::read_line();

    let range: Vec<&str> = input.split('-').collect();
    let lower = u32::from_str_radix(range[0], 10)
        .unwrap_or_else(|_| panic!("invalid lower bound: {}", range[0]));
    let upper = u32::from_str_radix(range[1], 10)
        .unwrap_or_else(|_| panic!("invalid upper bound: {}", range[1]));

    let passwords: Vec<u32> = (lower..=upper).collect();
    let passwords: Vec<&u32> = passwords
        .iter()
        .filter(|password| password.has_adjacent_group() && password.always_increases())
        .collect();

    println!("[PART ONE] number of passwords: {}", passwords.len());

    let passwords: Vec<&&u32> = passwords
        .iter()
        .filter(|password| password.has_adjacent_pair())
        .collect();

    println!("[PART TWO] number of passwords: {}", passwords.len());
}
