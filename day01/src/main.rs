extern crate aoc2018;

use std::collections::HashSet;

fn main() {
    let input: Vec<isize> = aoc2018::input(1).lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    
    // Part 1
    println!("Resulting frequency: {}", input.iter().sum::<isize>());

    // Part 2
    let mut seen = HashSet::new();
    seen.insert(0);
    
    let mut curr = 0;

    input.iter().cycle()
        .take_while(|x| {
            curr += **x;
            seen.insert(curr)
        }).last();

    println!("First frequency reached twice: {}", curr);

}
