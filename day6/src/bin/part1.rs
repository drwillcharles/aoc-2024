use std::fs;

struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &String) -> Self {
        let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            width,
            height,
        }
    }

    fn print(&self) {
        for row in 0..self.width {
            for col in 0..self.height {
                print!("{}",self.tiles[row][col])
            }
            print!("\n")
        }
    }
}

fn main() {
    let binding = fs::read_to_string("src/test.txt").unwrap();

    let grid = Grid::new(&binding);

    grid.print();
}

