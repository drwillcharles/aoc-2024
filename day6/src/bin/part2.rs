use std::fs;
// use std::collections::HashSet;

fn find_character(tiles: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    tiles.iter().enumerate().find_map(|(row_idx, row)| {
        row.iter()
            .position(|&ch| matches!(ch, '<' | '>' | '^' | '↓'))
            .map(|col_idx| (row_idx, col_idx))
    })
}
#[derive(Clone)]
struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
    guard_location: (usize, usize),
    guard_direction: Direction, // '<' | '>' | '^' | '↓'
    visited_obstacles: Vec<(usize, usize, Direction)>,
    in_loop: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Grid {
    fn new(input: &String) -> Self {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = tiles.len();
        let width = tiles[0].len();
        let guard_location = find_character(&tiles).unwrap_or((0, 0));
        let guard_direction = Direction::Up;
        Self {
            tiles,
            width,
            height,
            guard_location: guard_location, // Handle None if necessary,
            guard_direction: guard_direction,
            visited_obstacles: Vec::new(),
            in_loop: false,
        }
    }

    fn find_character(&self) -> Option<(usize, usize)> {
        return find_character(&self.tiles);
    }

    fn next_position(&mut self) -> Option<(usize, usize)> {
        let (row, col) = self.guard_location;
        // Convert to isize for safe arithmetic
        let (row, col) = (row as isize, col as isize);

        let next_position = match self.guard_direction {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        };

        // Check if the next position is within bounds
        let (next_row, next_col) = next_position;
        if next_row >= 0
            && next_row < self.height as isize
            && next_col >= 0
            && next_col < self.width as isize
        {
            // Convert back to usize and return
            Some((next_row as usize, next_col as usize))
        } else {
            None // Return None if out of bounds
        }
    }

    fn rotate_guard(&mut self) {
        match self.guard_direction {
            Direction::Up => self.guard_direction = Direction::Right,
            Direction::Right => self.guard_direction = Direction::Down,
            Direction::Down => self.guard_direction = Direction::Left,
            Direction::Left => self.guard_direction = Direction::Up,
        }
    }

    fn step(&mut self) -> bool {
        // First check if the guard has fallen off the grid
        // TODO

        let (current_row, current_col) = self.guard_location;
        if let Some((next_row, next_col)) = self.next_position() {
            // If the guard falls off the grid, it will be here
            // let next_tile = self.tiles[next_row][next_col];
            if let Some(value) = self
                .tiles
                .get(next_row)
                .and_then(|row| row.get(next_col))
                .copied()
            {
                let next_tile = value;

                // Move forward
                if next_tile == '.' || next_tile == 'X' {
                    self.guard_location = (next_row, next_col);
                    self.tiles[current_row][current_col] = 'X';
                    self.tiles[next_row][next_col] = '^';
                }

                // Rotate
                if next_tile == '#' || next_tile =='O' {
                    let direction = self.guard_direction.clone();
                    if self.visited_obstacles.contains(&(current_row, current_col, direction)) {
                        self.in_loop = true;
                        return false;
                    }
                    self.visited_obstacles.push((current_row, current_col, direction));
                    self.rotate_guard();
                }

                return true;
            } else { // Guard has exited the grid
                // Replace guard with X
                self.tiles[current_row][current_col] = 'X';

                return false;
            }
        } else { // Guard has exited the grid
            // Replace guard with X
            self.tiles[current_row][current_col] = 'X';

            return false;
        }

        
    }

    fn count_x(&self) -> usize {
        self.tiles.iter()
            .flat_map(|row| row.iter()) // Flatten the 2D grid into a single iterator
            .filter(|&&ch| ch == 'X')   // Filter only the 'X' characters
            .count()                    // Count the filtered items
    }

    fn print(&self) {
        for row in 0..self.width {
            for col in 0..self.height {
                print!("{}", self.tiles[row][col])
            }
            print!("\n")
        }
        print!("\n")
    }
}

fn main() {
    let binding = fs::read_to_string("src/input.txt").unwrap();

    let original_grid = Grid::new(&binding);
    let mut loop_count = 0;

    original_grid.print();

    // Need to save the original grid
    for i in 0..original_grid.height {
        for j in 0..original_grid.width {
            
            let mut grid = original_grid.clone();
            if grid.tiles[i][j] != '^' {
                grid.tiles[i][j] = 'O';
                while grid.step() {
                    // Debug
                    // grid.print();
                }
                if grid.in_loop {
                    loop_count += 1;
                }
            }
        }
    }

    println!("{:?}", loop_count)
}
