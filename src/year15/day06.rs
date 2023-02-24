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
        let start_x: usize = start[0].parse().unwrap();
        let start_y: usize = start[1].parse().unwrap();
        let end_x: usize = end[0].parse().unwrap();
        let end_y: usize = end[1].parse().unwrap();
        if toggle {
            toggle_vec(start_x, end_x, start_y, end_y, &mut light_vec, &mut light_vec_part_2);
        }
        else {
             change_vec(start_x, end_x, start_y, end_y, change_value, &mut light_vec, &mut light_vec_part_2);
        }
    }
    let answer1: usize = sum_vec(light_vec);
    let answer2: usize = sum_vec(light_vec_part_2);

    //print answers.
   println!("Answer 1 = {answer1}");
   println!("Answer 2 = {answer2}");

}

fn change_vec(start_x: usize, end_x: usize, start_y: usize, end_y: usize, change_value: usize, light_vec: &mut [Vec<usize>], light_vec_part_2: &mut [Vec<usize>]) {
    //this function turns the vector into 1 or 0; 
    //for answer two it adds or subtracts
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            light_vec[i][j] = change_value;
            if change_value == 1 {
                light_vec_part_2[i][j] += 1;
            } else if let Some(result) = light_vec_part_2[i][j].checked_sub(1) {
                    light_vec_part_2[i][j] = result;
            } else {
                light_vec_part_2[i][j] = 0;
            }
            
        }
    }
}

fn toggle_vec(start_x: usize, end_x: usize, start_y: usize, end_y: usize, light_vec: &mut [Vec<usize>], light_vec_part_2: &mut [Vec<usize>]) {
    //this function runs toggle commands
    //for answer 1 it swaps 0 and 1
    //for answer 2 it increases the value by 2
    for i in start_x..=end_x {
        for j in start_y..=end_y {
            // bitwise 1 XOR value = opposite of 1 or 0. XOR is ^ in rust.
            light_vec[i][j] ^= 1;
            light_vec_part_2[i][j] += 2;
        }
    }
}

fn sum_vec(vec_array: Vec<Vec<usize>>) -> usize {
    //this function just adds all the values in the array up.
    let mut answer = 0;
    for row in vec_array {
        for col in row {
            answer+=col;
        }
    }
    answer
}


