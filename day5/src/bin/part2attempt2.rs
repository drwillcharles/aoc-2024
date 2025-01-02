use std::fs;
// use std::collections::HashMap;
use std::{cmp::Ordering, collections::HashSet};



fn main() {
    let binding = fs::read_to_string("src/input.txt").unwrap();

    let parts: Vec<&str> = binding.split("\n\n").collect();

    // if parts.len() != 2 {
    //     println!("Error: Expected exactly two parts separated by '|'");
    //     return Ok(());
    // }

    let part1 = parts[0].to_string();
    let part2 = parts[1].to_string();

    let orderings: HashSet<(usize, usize)> = part1
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            Ordering::Less
        } else if orderings.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let total: usize = part2
        .lines()
        .map(|update| {
            let mut update: Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();

            if update.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
                0
            } else {
                update.sort_by(compare);
                update[update.len() / 2]
            }
        })
        .sum();

    println!("{:?}", total);
}
