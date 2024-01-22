#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use rand::Rng;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::{thread, time};
use toml::Value;

struct Config {
    rows: i64,
    cols: i64,
    timeout: i64,
}

impl Config {
    fn load(filename: &str) -> Result<Config, Box<dyn Error>> {
        // Load settings from config file

        let config_content = if let Ok(content) = fs::read_to_string(filename) {
            content
        } else {
            println!( "Error reading `{filename}` file. Using default settings.");
            String::from("rows = 20\n cols = 20\n timeout = 500")
        };

        let config: Value = config_content.parse()?;
        // Extract values from the config
        let conf = Config {
            rows: config["rows"].as_integer().unwrap_or(20),
            cols: config["cols"].as_integer().unwrap_or(20),
            timeout: config["timeout"].as_integer().unwrap_or(500),
        };
        Ok(conf)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let conf = Config::load("config.toml")?;
    let mut generation :u64  = 0;

    let mut grid = initialize_grid(&conf);

    loop {
        generation += 1;
        print_grid(&generation, &grid);
        update_grid(&mut grid);
        thread::sleep(time::Duration::from_millis(conf.timeout as u64));
    }
    // Ok(())
}

// Function to initialize the grid with random and predefined patterns
fn initialize_grid(conf: &Config) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; conf.cols as usize]; conf.rows as usize];
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
    for row in &mut grid {
        for cell in row.iter_mut() {
            *cell = rng.gen_bool(0.2); // Adjust the probability as needed
        }
    }
    grid
}

// Function to print the current state of the grid over the previous state
fn print_grid(generation: &u64, grid: &Vec<Vec<bool>>) {
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
    println!("Generation: {generation}");
    // Flush the output to ensure it is displayed immediately
    io::stdout().flush().unwrap();
}

fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut new_grid = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let neighbors = count_neighbors(grid, i as isize, j as isize);

            new_grid[i][j] = if grid[i][j] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }
    *grid = new_grid;
}

fn count_neighbors(grid: &[Vec<bool>], row: isize, col: isize) -> usize {
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len()  as isize;

    for i in (row - 1)..=(row + 1) {
        for j in (col - 1)..=(col + 1) {
            if     i >= 0
                && j >= 0
                && i < rows
                && j < cols
                && !(i == row && j == col)
                && grid[i as usize][j as usize]
            {
                count += 1;
            }
        }
    }

    count
}
