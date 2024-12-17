// use std::env;
use std::fs;

fn main() {

    // Read input
    let binding = fs::read_to_string("src/input.txt").unwrap();
    
    let mut total = 0;

    // Convert into two lists
    for line in binding
        .lines() {
            let mut split = line.split_whitespace();
            // let mut report = Vec::<i32>::new();
            let mut level_before: i32;
            let mut level_after: i32;
            let mut safe: bool;

            // Sorted ascending (a), descending(d) or NA (n)
            let mut sort: char = 'n';

            level_after = split.next().expect("No value found").parse().expect("Cannot be converted to integer");

            safe = true;
            for level in split {
                level_before = level_after;
                level_after = level.parse().expect("Cannot be converted to integer");

                if sort == 'n' {
                    if level_after > level_before {
                        sort = 'a'
                    } else {
                        sort = 'd'
                    }
                }
                
                if sort == 'a' {
                    if level_after - level_before < 1  || level_after - level_before > 3 {
                        safe = false;
                    }
                } else {
                    
                    if level_before - level_after < 1  || level_before - level_after > 3 {
                        safe = false;
                    }
                }
                
            }
            
            if safe {
                total += 1
            }

            // report.push(integer);
        }
        
    // Print output 
    // println!("With text:\n{:?}", input1);
    // println!("With text:\n{:?}", input2);
    println!("{:?}", total)
}