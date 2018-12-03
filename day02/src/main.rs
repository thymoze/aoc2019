extern crate aoc2018;

use std::collections::HashMap;

fn main() {
    let input = aoc2018::input(2);

    println!("Checksum: {}", part_one(&input));

    println!("Same chars: {}", part_two(&input).unwrap_or("None".to_owned()));

}

fn part_two(input: &String) -> Option<String> {

    // a = 97
    // z = 122
    // difference < 25

    let sum_map = input.lines()
        .map(|l| (l.to_owned(), l.chars().map(|c| (c as i64)).sum::<i64>()))
        //.inspect(|x| println!("{:?}", x))
        .collect::<HashMap<String, i64>>();

    let mut winning_comb = None;
    let mut index = None;

    for (k, v) in &sum_map {
        let mut similar = Vec::new();

        for (kk, vv) in &sum_map {
            if k == kk { continue; }
            if (v - vv).abs() < 26 { 
                similar.push(kk);
            }
        }
        let mut key = None;

        for kk in similar {
            let mut diff = 0;
            for (i, (c, cc)) in k.chars().zip(kk.chars()).enumerate() {
                if c != cc {
                    diff += 1;
                    index = Some(i);
                }
            }
            if diff < 2 {
                key = Some(kk);
                break;
            }
        }

        if let Some(kk) = key {
            
            winning_comb = Some((k.clone(), kk.clone()));
            break;
        }
    }


    if winning_comb.is_some() {
        let (mut k, mut kk) = winning_comb.unwrap();

        k.remove(index.unwrap());
        kk.remove(index.unwrap());

        assert_eq!(k, kk);
        return Some(k);
    }
    None
}

fn part_one(input: &String) -> i32 {
    let (twice, thrice) = input.lines()
        .map(|s| s.chars())
        .fold((0, 0), |(two, three), chars| {
            let mut map: HashMap<char, i32> = HashMap::new();

            chars.for_each(|c| {
                let val = map.get(&c).unwrap_or(&0) + 1;
                map.insert(c, val); 
            });

            let (tw, thr) = map.iter().filter(|(_, v)| v > &&1)
                //.inspect(|x| print!("{:?}", x))
                .fold((0, 0), |(tw, thr), (_, v)| {
                    if v > &2 {
                        (tw, 1)
                    } else { 
                        (1, thr) 
                    }
                });
            //println!("\t{:?}", (tw, thr));
            (two + tw, three + thr)
        });

    twice * thrice
}