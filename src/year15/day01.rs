use std::fs::read_to_string;

pub fn day01() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input01.txt"#).expect("Failed to read file");
    //initialize floor count and answer2
    let mut floor: i32 = 0;
    let mut count = 0;
    let mut answer2 = 0;
    //loop through characters of input data
    for character in input.chars() {
        //change floors based on character
        match character {
            ')' => {floor -= 1},
            '(' => {floor += 1},
            _ => {},
        }
        //increase count for answer 2
        count+=1;
        //if floor is basement and answer2 hasn't changed, we are on first pass. modify answer2 to get correct answer.
        if floor == -1 && answer2 == 0{
            answer2 = count;
            // adding break made code 13.3x faster
            break;
        }
    }
    //this just counts the values, and sums them. this is 13x faster than looping and adding all values.
    floor = (input.matches('(').count() as i32) - (input.matches(')').count() as i32);
    //print answers.
    println!("Answer 1 = {floor}");
    println!("Answer 2 = {answer2}");
}
