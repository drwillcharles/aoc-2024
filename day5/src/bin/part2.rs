use std::fs;
// use std::collections::HashMap;
use multimap::MultiMap;

fn create_map(binding: &String) -> MultiMap<i32, i32> {
    let mut after_before = MultiMap::new();

    for line in binding.lines() {
        // println!("{}", line);

        // // Once the first section is finished. Will need to add a bit for the next section
        // // Not needed anymore
        // if line.len() == 0 {
        //     break
        // }

        let mut parts = line.split('|');
        let before = parts.next().unwrap().parse::<i32>().unwrap();
        let after = parts.next().unwrap().parse::<i32>().unwrap();
        after_before.insert(after, before);
    }

    return after_before;
}

fn create_updates(binding: &String) -> Vec<Vec<i32>> {
    let mut updates = Vec::new();

    for line in binding.lines() {
        updates.push(line.split(',').map(|n| n.parse().unwrap()).collect())
    }

    return updates;
}

fn check_order(
    after_before: &MultiMap<i32, i32>,
    update: &mut Vec<i32>,
    i: &usize,
    val: &i32,
) -> bool {
    let mut valid: bool = true;
    let check = after_before.get_vec(val);
    // Using pattern matching (recommended)
    match check {
        Some(number) => {
            // println!("The number is: {}", number);
            // Perform your action with the number here
            for before in number {
                if update[*i..].contains(before) {
                    //Change to while loop
                    println!("Invalid {:?} in {:?}", number, update);
                    // Fix value
                    update.retain(|&x| x != *val);
                    update.insert(i - 1, *before);

                    valid = false;
                }
            }
        }
        None => {
            // println!("The number is not present.");
            // return true
        }
    }
    return valid;
}
fn check_updates(after_before: &MultiMap<i32, i32>, updates: &mut Vec<Vec<i32>>) -> i32 {
    // Now we check the rules
    let mut total: i32 = 0;

    for mut update in updates.iter_mut() {
        let mut valid: bool;
        for (i, val) in update.iter_mut().enumerate() {
            // Check if page has appeared
            valid = check_order(&after_before, &mut update, &i, &val);

            // Don't think we need this for part 2
            if !valid {
                break;
            }

            if i == update.len() - 1 && valid {
                let middle = update[update.len() / 2];
                println!("Middle number is {:?}", middle);
                total += middle;
            }
        }
        // println!("{:?}",update);
    }
    return total;
}

fn main() {
    let binding = fs::read_to_string("src/test.txt").unwrap();

    let parts: Vec<&str> = binding.split("\n\n").collect();

    // if parts.len() != 2 {
    //     println!("Error: Expected exactly two parts separated by '|'");
    //     return Ok(());
    // }

    let part1 = parts[0].to_string();
    let part2 = parts[1].to_string();

    // // Debug
    // for line in part1.lines() {
    //     println!("part1{}", line);
    // }

    // for line in part2.lines() {
    //     println!("part2{}", line);
    // }

    // For the first section, create a hasmap of the page rules
    let after_before = create_map(&part1);

    // Now create lists of updates
    let mut updates = create_updates(&part2);

    let total = check_updates(&after_before, &mut updates);

    println!("{:?}", total);
}
