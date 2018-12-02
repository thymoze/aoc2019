#![feature(euclidean_division)]

extern crate aoc2018;

use std::hash::*;
use std::iter::*;
use std::collections::*;

#[derive(Debug)]
struct Map<K, V>(HashMap<K, Vec<V>>)
    where K: Eq + Hash, 
          V: Eq + std::cmp::Ord;

impl<K, V> FromIterator<(K, V)> for Map<K, V> 
    where K: Eq + Hash,
          V: Eq + std::cmp::Ord
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Map<K, V> {
        let mut map = HashMap::new();
        for (k, v) in iter {
            if map.contains_key(&k) {
                map.entry(k).and_modify(|ref mut set: &mut Vec<V>| { set.push(v); });
            } else {
                let mut set = Vec::new();
                set.push(v);
                map.insert(k, set);
            }
        }
        Map(map)
    }
}


fn main() {
    let input: Vec<isize> = aoc2018::input(1)
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    
    // Part 1
    println!("Resulting frequency: {}", input.iter().sum::<isize>());

    // Part 2
    println!("{:?}", part_two(&input));
    println!("{}", part_two_set(&input));
}

fn part_two(input: &Vec<isize>) -> Option<isize> {
    let mut iter0 = Vec::new();
    let total = input.iter().fold(0, |acc, x| {
        iter0.push(acc);
        let sum = acc + x;
        sum
    });

    let mut seen_set = HashSet::new();
    if total == 0 {
        return Some(0);
    } else if let Some(res) = iter0.iter().find_map(|x| seen_set.replace(x)) {
        return Some(*res);
    } else {
        let res = iter0.iter()
            .enumerate()
            .map(|(i, x)| (x.mod_euc(total), (i, x)))
            .collect::<Map<_, _>>().0.iter_mut()
            .filter(|(_, v)| v.len() > 1)
            .map(|(_, vec)| { vec.sort_by_key(|(_, x)| *x); vec })
            .map(|vec| {
                vec.windows(2)
                    .fold((None, None, None), |(mut min_diff, mut min_index, mut min_freq), window| {
                        if let &[(i, x), (j, y)] =  window { 
                            let diff = y - x;
                            let index = if total > 0 { i } else { j };
                            let freq = if total > 0 { y } else { x };

                            if min_diff.map_or(true, |x| diff < x || (diff == x && Some(index) < min_index)) {
                                min_diff.replace(diff);
                                min_index.replace(index);
                                min_freq.replace(freq);
                            }
                        }
                        (min_diff, min_index, min_freq)
                    })
            })
            .fold((None, None, None), |(min_diff, min_index, min_freq), (diff, index, freq)| {
                if min_diff.map_or(true, |x| diff < x || (diff == x && Some(index) < min_index)) {
                    (Some(diff), Some(index), Some(freq))
                } else {
                    (min_diff, min_index, min_freq)
                }
            }).2.unwrap_or(None);

        return res.cloned();
    }
}

fn part_two_set(input: &Vec<isize>) -> isize {
    let mut seen = HashSet::new();
    seen.insert(0);
    
    let mut curr = 0;

    input.iter().cycle()
        .take_while(|x| {
            curr += **x;
            seen.insert(curr)
        }).last();

    curr
}
