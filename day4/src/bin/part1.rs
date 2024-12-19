// use std::env;
use std::fs;
use regex::Regex;

fn file_to_grid(filename: &str) -> Vec<Vec<char>> {
    let binding = fs::read_to_string(filename).unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line_result in binding.lines() {
        let line = line_result;
        let row: Vec<char> = line.chars().collect(); // Convert the line to a Vec<char>
        grid.push(row);
    }

    grid // Return the grid if successful
}

fn grid_to_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut combinations: Vec<String> = Vec::new();

    // Across
    for (_, row) in grid.iter().enumerate() {
        let mut current_row: String = String::new();
        for (_, &char) in row.iter().enumerate() {
            current_row.push(char);
        }
        combinations.push(current_row);
    }

    // Down
    for (y, row) in grid.iter().enumerate() {
        let mut current_row: String = String::new();
        for (x, _) in row.iter().enumerate() {
            current_row.push(grid[x][y]);
        }
        combinations.push(current_row);
    }

    // Diagonal up and down. Reverse can be handled with regex SAMX
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // let mut main_diagonals: Vec<String> = Vec::new();
    // let mut anti_diagonals: Vec<String> = Vec::new();

    // Main diagonals
    for k in 0..num_rows + num_cols - 1 {
        let mut diagonal: Vec<char> = Vec::new();
        for i in 0..num_rows {
            let j = k as i32 - i as i32;
            if j >= 0 && j < num_cols as i32 {
                diagonal.push(grid[i][j as usize]);
            }
        }
        if !diagonal.is_empty() {
            combinations.push(diagonal.into_iter().collect());
        }
    }

    // Anti-diagonals
    for k in 0..num_rows + num_cols - 1 {
        let mut diagonal: Vec<char> = Vec::new();
        for i in 0..num_rows {
            let j = k as i32 - (num_rows as i32 - 1) + i as i32;
            if j >= 0 && j < num_cols as i32 {
                diagonal.push(grid[i][j as usize]);
            }
        }
        if !diagonal.is_empty() {
            combinations.push(diagonal.into_iter().collect());
        }
    }

    return combinations
}

fn search_regex(search_str: &str, combinations: &Vec<String>) -> usize {
    let re = Regex::new(search_str).unwrap();
    let mut total = 0;
    for line in combinations {
        let count = re.find_iter(&line).count();
        // // Debug
        // println!{"{}: {}",count, line}
        total += count;
    }
    return total
}
fn main() {
    // Read input
    let grid = file_to_grid("src/input.txt");

    // // Check grid
    // for (_, row) in grid.iter().enumerate() {
    //     for (_, c) in row.iter().enumerate() {
    //         print!("{:?} ", c);
    //     }
    //     println!(); // Newline after each row
    // }

    // ?? Check we are happy with the grid system
    // println!("{:?}", grid[3][2]);  // Format x, y

    // Now add across, down and diagonal to possible combinations
    let combinations = grid_to_lines(&grid);
    
    // Use Regex to obtain numbers
    let mut total = 0;
    total += search_regex("XMAS", &combinations);
    total += search_regex("SAMX", &combinations);

    // println!("test")
    // println!("{}", combinations.join("\n"))
    println!("{}", total);
}