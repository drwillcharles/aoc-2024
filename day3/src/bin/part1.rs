// use std::env;
use std::fs;
use regex::Regex;

fn main() {
    // Read input
    let binding = fs::read_to_string("src/input.txt").unwrap();

    // Use Regex to obtain numbers
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    let mut total = 0;
    for cap in re.captures_iter(&binding) {
        let i1_str = cap.get(1).unwrap().as_str();
        let i2_str = cap.get(2).unwrap().as_str();

        // Handle potential parsing errors
        let i1 = i1_str.parse::<i32>();
        let i2 = i2_str.parse::<i32>();

        if let (Ok(i1), Ok(i2)) = (i1, i2) {
            total += i1 * i2;
        } else {
            eprintln!("Error parsing numbers: {} {}", i1_str, i2_str);
        }
    }

    // println!("test")
    println!("{}", total)
}