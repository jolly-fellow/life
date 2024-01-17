use rand::Rng;
use std::{thread, time};
use std::io::{self, Write};
use std::fs;
use toml::Value;

const ROWS: usize = 60;
const COLS: usize = 60;
const INTERVAL: u64 = 600; // milliseconds

fn main() {
    let mut grid = initialize_grid();

    loop {
        print_grid(&grid);
        update_grid(&mut grid);
        thread::sleep(time::Duration::from_millis(INTERVAL));
    }
}

// Function to initialize the grid with random and predefined patterns
fn initialize_grid() -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; COLS]; ROWS];
    let mut rng = rand::thread_rng();

    // Glider pattern
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;

    // Blinker pattern
    grid[7][9] = true;
    grid[7][10] = true;
    grid[7][11] = true;

    // Randomize the remaining cells
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = rng.gen_bool(0.2); // Adjust the probability as needed
        }
    }

    grid
}


// Function to print the current state of the grid over the previous state
fn print_grid(grid: &Vec<Vec<bool>>) {
    // ANSI escape code to move the cursor to the top-left corner
    print!("\x1B[H");

    for row in grid {
        for &cell in row {
            if cell {
                print!("■ ");
            } else {
                print!("□ ");
            }
        }
        println!();
    }
    println!();
    // Flush the output to ensure it is displayed immediately
    io::stdout().flush().unwrap();
}

fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = vec![vec![false; COLS]; ROWS];

    for i in 0..ROWS {
        for j in 0..COLS {
            let neighbors = count_neighbors(grid, i, j);

            new_grid[i][j] = if grid[i][j] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }

    *grid = new_grid;
}

fn count_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if i >= 0 && i < ROWS as isize && j >= 0 && j < COLS as isize {
                if !(i == row as isize && j == col as isize) && grid[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}
