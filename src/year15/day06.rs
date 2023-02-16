use std::fs::read_to_string;

pub fn day06() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input06.txt"#).expect("Failed to read file");
    let input_lines = input.split_terminator('\n');

    //build vectors for 2d arrays
    let rows = 1000;
    let cols = 1000;
    let mut light_vec = vec![vec![0; cols]; rows];
    let mut light_vec_part_2 = vec![vec![0; cols]; rows];

    for line in input_lines {
        //convert line into fields, and determine on/off/toggle
        let mut fields: Vec<&str> = line.split(' ').collect();
        let mut change_value = 0;
        let mut toggle = false;
        //if on change value to 1
        if fields[1] == "on" {change_value = 1;}
        // println!("{change}");
        //if toggle, add a field for uniformity and set toggle to true.
        if fields[0] == "toggle" {
            toggle = true;
            //insert nonsense so the line aligns with needed splits below
            fields.insert(0, "nonsense")
        }

        //convert all fields into individual values
        let start: Vec<&str> = fields[2].split(',').collect();
        let end: Vec<&str> = fields[4].split(',').collect();
        let start_x: i32 = start[0].parse().unwrap();
        let start_y: i32 = start[1].parse().unwrap();
        let end_x: i32 = end[0].parse().unwrap();
        let end_y: i32 = end[1].parse().unwrap();
        if toggle {
            toggle_vec(start_x, end_x, start_y, end_y, &mut light_vec, &mut light_vec_part_2);
        }
        else {
             change_vec(start_x, end_x, start_y, end_y, change_value, &mut light_vec, &mut light_vec_part_2);
        }
    }
    let answer1: i32 = sum_vec(light_vec);
    let answer2: i32 = sum_vec(light_vec_part_2);

    //print answers.
   println!("Answer 1 = {answer1}");
   println!("Answer 2 = {answer2}");

}

fn change_vec(start_x: i32, end_x: i32, start_y: i32, end_y: i32, change_value: i32, light_vec: &mut Vec<Vec<i32>>, light_vec_part_2: &mut Vec<Vec<i32>>) {
    //this function turns the vector into 1 or 0; 
    //for answer two it adds or subtracts
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            light_vec[i as usize][j as usize] = change_value;
            if change_value == 1 {
                light_vec_part_2[i as usize][j as usize] += 1;
            }
            else {
                light_vec_part_2[i as usize][j as usize] -= 1;
                if light_vec_part_2[i as usize][j as usize] < 0 {
                    light_vec_part_2[i as usize][j as usize] = 0;
                }
            }
        }
    }
}

fn toggle_vec(start_x: i32, end_x: i32, start_y: i32, end_y: i32, light_vec: &mut Vec<Vec<i32>>, light_vec_part_2: &mut Vec<Vec<i32>>) {
    //this function runs toggle commands
    //for answer 1 it swaps 0 and 1
    //for answer 2 it increases the value by 2
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            // bitwise 1 XOR value = opposite of 1 or 0. XOR is ^ in rust.
            light_vec[i as usize][j as usize] = 1 ^ light_vec[i as usize][j as usize];
            light_vec_part_2[i as usize][j as usize] += 2;
        }
    }
}

fn sum_vec(vec_array: Vec<Vec<i32>>) -> i32 {
    //this function just adds all the values in the array up.
    let rows = vec_array.len();
    let cols = vec_array[0].len();
    let mut answer = 0;
    for i in 0..rows {
        for j in 0..cols {
            answer += vec_array[i as usize][j as usize];
        }
    }
    answer
}


