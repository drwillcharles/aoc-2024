// use std::env;
use std::fs;

fn main() {
    let mut input1 = Vec::<i32>::new();
    let mut input2 = Vec::<i32>::new();

    // Read input
    let binding = fs::read_to_string("src/day1/input.txt").unwrap();

    // Convert into two lists
    for line in binding
        .lines() {
            let mut split = line.split_whitespace();
            let integer1 = split.next().expect("REASON").parse().expect("Cannot be converted to integer");
            let integer2 = split.next().expect("REASON").parse().expect("Cannot be converted to integer");
            input1.push(integer1);
            input2.push(integer2);
        }
    
    //Sort lists
    input1.sort();
    input2.sort();

    // Find the difference between the two lists
    let mut total = 0;
    for (x, y) in input1.iter().zip(input2.iter()) {
        total += i32::abs(x-y);
    }

    // Convertlist of ints to list to strings
    // let strings: Vec<String> = input1.iter().map(|n| n.to_string()).collect();
    
    // Print output 
    // println!("With text:\n{:?}", input1);
    // println!("With text:\n{:?}", input2);
    println!("{}", total)
}