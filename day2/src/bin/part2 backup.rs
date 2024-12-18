// use std::env;
use std::fs;

fn compare_two_levels(level_before: i32, level_after: i32, sort: char) -> bool {
    if sort == 'a' {
        if level_after - level_before < 1  || level_after - level_before > 3 {
            return false
        }
    } else {
        if level_before - level_after < 1  || level_before - level_after > 3 {
            return false
        }
    }

    return true
}


fn determine_sort(level_before:i32, level_after: i32, sort: char) -> char {
    if sort == 'n' {
        if level_after > level_before {
            return 'a'
        } else {
            return 'd'
        }
    };

    return sort;
}

fn main() {

    // Read input
    let binding = fs::read_to_string("src/test.txt").unwrap();
    
    let mut total = 0;

    // Convert into two lists
    for line in binding
        .lines() {
            let mut split = line.split_whitespace();
            // let mut report = Vec::<i32>::new();
            let mut level_before: i32;
            let mut level_after: i32;
            let mut level_errors: i32;

            // Sorted ascending (a), descending(d) or NA (n)
            let mut sort: char = 'n';

            level_after = split.next().expect("No value found").parse().expect("Cannot be converted to integer");

            
            let mut temp_level: i32 = -1;

            level_errors = 0;
            for level in split {

                level_before = level_after;
                level_after = level.parse().expect("Cannot be converted to integer");

                // Need to consider the case when there is an error on the second level
                sort = determine_sort(level_before, level_after, sort);
                
                if !compare_two_levels(level_before, level_after, sort) {

                    // Case when the first number should be removed, not the second
                    if level_errors == 1 {
                        level_before = temp_level;
                        sort = determine_sort(level_before, level_after, 'n');
                        if compare_two_levels(level_before, level_after, sort) {
                            continue;
                        }
                    }
                    
                    level_errors += 1;
                    temp_level = level_after;
                    level_after = level_before;
                    sort = 'n'

                }

                
            }
            
            if level_errors < 2 {
                total += 1
            }

            // report.push(integer);
            // Debug
            print!("{:?}", line);
            println!("{:?}", total);
        }
        
    // Print output 
    // println!("With text:\n{:?}", input1);
    // println!("With text:\n{:?}", input2);
    println!("{:?}", total)
}