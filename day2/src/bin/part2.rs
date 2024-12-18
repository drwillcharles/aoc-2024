// use std::env;
use std::fs;

// fn compare_two_levels(level_before: i32, level_after: i32, sort: char) -> bool {
//     if sort == 'a' {
//         if level_after - level_before < 1  || level_after - level_before > 3 {
//             return false
//         }
//     } else {
//         if level_before - level_after < 1  || level_before - level_after > 3 {
//             return false
//         }
//     }

//     return true
// }


// fn determine_sort(numbers: Vec<u64>) -> char {
//     for i in numbers.len() {

//     }

//     return sort;
// }

fn is_safe_increasing(pair: &[i32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0] > 0) && (pair[1] - pair[0]) < 4
}

fn is_safe_decreasing(pair: &[i32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1] > 0) && (pair[0] - pair[1]) < 4
}

fn is_safe_with_dampener(report: Vec<i32>) -> i32 {

    for i in 0..report.len() {
        let inc = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_increasing);
        let dec = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_decreasing);

        if inc || dec {
            return 1;
        }
    }
    return 0;
}

fn main() {

    // Read input
    let binding = fs::read_to_string("src/input.txt").unwrap();
    
    let mut total = 0;

    // Convert into two lists
    for line in binding
        .lines() {
            let numbers: Vec<i32> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            total += is_safe_with_dampener(numbers);

            // Debug
            print!("{:?}", line);
            println!("{:?}", total);
        }

    println!("{:?}", total)
}