extern crate aoc2018;

use std::collections::HashSet;

fn main() {
    let input: Vec<i32> = aoc2018::input(1).lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    
    // Part 1
    println!("Resulting frequency: {}", input.iter().sum::<i32>());

    // Part 2
    let mut seen: HashSet<i32> = HashSet::with_capacity(input.len());
    let mut curr_freq = 0;
    seen.insert(curr_freq);

    for change in input.iter().cycle() {
        curr_freq += change;

        if !seen.insert(curr_freq) {
            break;
        }
    }
    println!("First frequency reached twice: {}", curr_freq);

}
