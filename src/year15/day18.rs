use std::fs::read_to_string;

pub fn day18() {
    let input: String  = read_to_string(r#"c:\tools\adventofcode\2015\input18.txt"#).expect("Failed to read file.");
    // let input: String = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..".to_string();
    let split_input: Vec<&str> = input.split_terminator('\n').collect();
    let width: usize = split_input[0].len();
    let height: usize = split_input.len();
    //build starting grid from scratch
    let mut actual_grid: Vec<Vec<bool>> = build_starting_grid(height, split_input.clone(), width);
    //use secondary grid, so changes don't effect our grid per round
    //in other words, make changes based on original grid,
    let mut working_grid: Vec<Vec<bool>> = actual_grid.clone();
    //dont enforce cornerlights stay on
    let mut corner_lights = false;
    calculate_grid(height, width, &mut actual_grid, &mut working_grid, corner_lights);
    let answer1: i32 = calculate_lights_on(height, width, actual_grid);

    //rebuild grid from starting position
    let mut actual_grid: Vec<Vec<bool>> = build_starting_grid(height, split_input.clone(), width);
    let mut working_grid: Vec<Vec<bool>> = actual_grid.clone();
    //enforce cornerlights stay on
    corner_lights = true;
    //find new final grid
    calculate_grid(height, width, &mut actual_grid, &mut working_grid, corner_lights);
    //get final number of lights on
    let answer2: i32 = calculate_lights_on(height, width, actual_grid);

    println!("Answer 1 = {}", answer1);
    println!("Answer 2 = {}", answer2);
}

fn calculate_lights_on(height: usize, width: usize, actual_grid: Vec<Vec<bool>>) -> i32 {
    //loop through all grid items and sum the number of lights on.
    let mut answer1 = 0;
    for i in 0..height {
        for j in 0..width {
            if actual_grid[i][j] {
                answer1+=1;
            }
        }
    }
    answer1
}

fn build_starting_grid(height: usize, split_input: Vec<&str>, width: usize) -> Vec<Vec<bool>> {
    //simple function that makes a grid based on height and widht of split_input
    let mut actual_grid: Vec<Vec<bool>> =  vec![vec![false; width]; height];
    for line in 0..height {
        let line_vec: Vec<char> = split_input[line].to_string().chars().collect();
        for i in 0..width {
            actual_grid[line][i] = line_vec[i] == '#';
        }
    }
    actual_grid
}

fn calculate_grid(height: usize, width: usize, actual_grid: &mut Vec<Vec<bool>>, working_grid: &mut Vec<Vec<bool>>, corner_lights: bool) {
    // loop through all light changes (puzzle asked for 100 loops)
    for _ in 0..100 {
        //get grid points
        for i in 0..height {
            for j in 0..width {
                //find all neighbors
                let neighbor_count: i32 = get_num_of_neighbors((i as i32, j as i32), &*actual_grid);
                //make changes to working grid based on actual grid
                if actual_grid[i][j] && (neighbor_count ==2 || neighbor_count == 3) {
                    //don't do anything if light is on and neighbor count is 2 or 3
                    continue;
                }   else {
                    //otherwise if light is on, but different neighbor count, change to off
                    working_grid[i][j] = false;
                }
            
                if !actual_grid[i][j] && (neighbor_count == 3) {
                    //if light is off, only turn on if it has 3 neighbors.
                    working_grid[i][j] = true;
                }

            }
            //if cornerlights are enforced on, we ensure they are on each loop.
            if corner_lights {
                working_grid[0][0] = true;
                working_grid[0][width-1] = true;
                working_grid[height-1][0] = true;
                working_grid[height-1][width-1] = true;
            }
        }
        *actual_grid = working_grid.clone();
    }
}
fn get_num_of_neighbors(position: (i32, i32), actual_grid: &Vec<Vec<bool>>) -> i32 {
    //just find neighbors
    let (row, col) = position;
    let mut count: i32 = 0;
    //loop through all 8 neighbors
    //do to negative numbers and system desire to be usize,
    //we use i32 or convert to i32 in most places.
    //usize is only used when necessary
    for i in -1..=1 {
        for j in -1..=1 {
            //don't count self
            if i == 0 && j == 0 {
                continue; // ignore current cell
            }
            //
            let x: i32 = row + i;
            let y: i32 = col + j;
            if x < 0 || y < 0 {
                continue; // outside grid, ignore
            }
            let row_len = actual_grid.len();
            if x as usize >= row_len {
                continue; // outside grid, ignore
            }
            let col_len = actual_grid[x as usize].len();
            if y as usize >= col_len {
                continue; // outside grid, ignore
            }
            if actual_grid[x as usize][y as usize] {
                count += 1;
            }
        }
    }
    count
}