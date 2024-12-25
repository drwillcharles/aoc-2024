use std::fs;
use std::collections::HashMap;

fn create_map(binding :&String) -> HashMap<i32,i32> {
    let mut after_before = HashMap::new();

    for line in binding.lines() {
        println!("{}", line);

        // Once the first section is finished. Will need to add a bit for the next section
        if line.len() == 0 {
            break
        }

        let mut parts = line.split('|');
        let before = parts.next().unwrap().parse::<i32>().unwrap();
        let after = parts.next().unwrap().parse::<i32>().unwrap();
        after_before.insert(after, before);
    }

    return after_before
}

fn create_updates(binding: &String) -> Vec<Vec<i32>> {
    let mut updates = Vec<Vec<i32>>::new();
}

fn main() {
    let binding = fs::read_to_string("src/test.txt").unwrap();

    // For the first section, create a hasmap of the page rules
    let after_before = create_map(&binding);

    // Now create lists of updates


}

