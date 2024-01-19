use rand::Rng;
use std::{thread, time};
use std::io::{self, Write};
use std::fs;
use toml::Value;
use std::error::Error;

struct Config {
    rows : usize,
    cols : usize,
    timeout : u64,
}

impl Config {

    fn load(filename: & str) -> Result<Config, Box<dyn Error>> {
        // Load settings from config file
        let config_content = fs::read_to_string(filename)?;
        let config: Value = config_content.parse()?;
        // Extract values from the config
        let conf = Config {
            rows : config["rows"].as_integer().unwrap_or(20) as usize,
            cols : config["cols"].as_integer().unwrap_or(20) as usize,
            timeout : config["timeout"].as_integer().unwrap_or(500) as u64,
        };
        Ok(conf)
    }
}

fn main() -> Result<(), Box<dyn Error>>  {

    let conf = Config::load("config.toml")?;

    let mut grid = initialize_grid(&conf);

    loop {
        print_grid(&grid);
        update_grid(&conf, &mut grid);
        thread::sleep(time::Duration::from_millis(conf.timeout));
    }
    Ok(())
}

// Function to initialize the grid with random and predefined patterns
fn initialize_grid(conf: & Config) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; conf.cols]; conf.rows];
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

fn update_grid(conf: &Config, grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = vec![vec![false; conf.cols]; conf.rows];

    for i in 0..conf.rows {
        for j in 0..conf.cols {
            let neighbors = count_neighbors(conf, grid, i, j);

            new_grid[i][j] = if grid[i][j] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }

    *grid = new_grid;
}

fn count_neighbors(conf: &Config, grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if i >= 0 && i < conf.rows as isize && j >= 0 && j < conf.cols as isize {
                if !(i == row as isize && j == col as isize) && grid[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}
