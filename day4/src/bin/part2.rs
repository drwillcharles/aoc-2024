use std::fs;

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


fn check_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    // Consider a smaller 3x3 grid with 'A' in the centre
    let mut diagonals: Vec<String> = Vec::new();

    let mut diagonal: Vec<char> = Vec::new();
    let mut idiagonal: Vec<char> = Vec::new();

    // l = [-1, 0, 1]
    for li in 1..4  {
        let l:i32 = li-2;
        diagonal.push(grid[(i as i32+l) as usize][(j as i32+l) as usize]);
        idiagonal.push(grid[(i as i32+l) as usize][(j  as i32-l) as usize]);
        // Debug
        print!("{}", grid[(i as i32+l) as usize][(j as i32+l) as usize]);
        print!("[{:?},{:?}]",i as i32+l,j as i32+l);
        print!("{}", grid[(i as i32+l) as usize][(j  as i32-l) as usize]);
        println!("[{:?},{:?}]",i as i32+l,j  as i32-l);
    }

    diagonals.push(diagonal.into_iter().collect());
    diagonals.push(idiagonal.into_iter().collect());


    for row in diagonals {
        // if row.into_iter().collect() in ["MAS","SAM"] {
        if !["MAS", "SAM"].iter().any(|&s| s == row) {
            return 0
        }
    }
    return 1
}

fn main() {
    // Read input
    let grid = file_to_grid("src/input.txt");

    
    // Method
    // - Find A and it's associated X,Y
    // - Using a subset of a grid, find the diagonal lines
    // - Check the diagonal lines are MAS or SAM

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut total = 0;

    // Find all the A's
    for i in 1..num_rows - 1 {
        for j in 1..num_cols - 1 {
            // // Debug
            // print!("{:?} ", grid[i][j]);
            if grid[i][j] == 'A' {
                total += check_mas(&grid, i, j);
            }
        }
        println!();
    }


    println!("{}", total);
}
