extern crate aoc2018;
extern crate regex;

use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = aoc2018::input(3);

    let mut map: HashMap<(u32, u32), bool> = HashMap::new();
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();


    // PART ONE
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        
        let start_x: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let start_y: u32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let width: u32 = cap.get(4).unwrap().as_str().parse().unwrap();
        let height: u32 = cap.get(5).unwrap().as_str().parse().unwrap();

        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                map.entry((x, y))
                    .and_modify(|v| { *v = true } )
                    .or_insert(false);
            }
        }
    }

    let count = map.iter().filter(|(_, v)| **v).count();

    println!("{:?}", count);


    // PART TWO
    let mut index: Option<u32> = None;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        
        let start_x: u32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let start_y: u32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let width: u32 = cap.get(4).unwrap().as_str().parse().unwrap();
        let height: u32 = cap.get(5).unwrap().as_str().parse().unwrap();

        let mut overlap = false;

        for y in start_y..start_y + height {
            for x in start_x..start_x + width {
                overlap = overlap || *map.get(&(x, y)).unwrap();
            }
        }

        if !overlap {
            index = Some(cap.get(1).unwrap().as_str().parse().unwrap());
            break;
        }
    }

    println!("{:?}", index);
}