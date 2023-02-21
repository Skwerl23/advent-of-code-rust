use std::fs::read_to_string;

pub fn day17() {
    let input  = read_to_string(r#"c:\tools\adventofcode\2015\input17.txt"#).expect("Failed to read file.");
    let split_input = input.split_terminator('\n');
    // let mut test_inputs: Vec<i32> = [20, 15, 10, 5, 5].to_vec();

    let mut inputs: Vec<i32> = Vec::new();
    for i in split_input {
        inputs.push(i.parse::<i32>().unwrap());
    }
    // test example results in 3
    // let mut combinations = Vec::new();
    // let mut max_value = 25;
    // let mut seen: Vec<i32> = Vec::new();
    // get_combinations(&test_inputs, &mut combinations, 0, max_value, &mut seen);
    // // 85145 is too high
    // //21532 is too high
    // println!("Test 1 = {}", combinations.len());


    let mut combinations = Vec::new();
    let max_value = 150;
    let mut seen = Vec::new();
    get_combinations(&inputs, &mut combinations, 0, max_value,&mut seen);
    let answer1 = combinations.len();
    println!("Answer 1 = {}", answer1);

    let answer2 = get_combinations_of_least_containers(combinations);
    println!("Answer 2 = {}", answer2);

}

fn get_combinations_of_least_containers(combinations: Vec<Vec<i32>>) -> i32 {
    let mut shortest: usize = usize::MAX;
    for i in &combinations {
        if i.len() < shortest {
            shortest = i.len() 
        }
    }
    let mut answer2 = 0;
    for i in &combinations {
        if i.len() == shortest {
            answer2 += 1;
        }
    }
    answer2
}


fn get_combinations(inputs: &[i32], combinations: &mut Vec<Vec<i32>>, current_sum: i32, max_value: i32, seen: &mut Vec<i32>) {
    let mut value: usize  = 0;
    for i in inputs {
        value+=1;
 
        seen.push(i.clone());
        if i+current_sum == max_value {
            combinations.push(seen.clone());
            // println!("{:?}", seen);
        }
        get_combinations(&inputs[value..], combinations, current_sum+i, max_value, seen);
        seen.pop();
        
    }
}