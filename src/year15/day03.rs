use std::fs::read_to_string;
use std::collections::HashSet;
pub fn day03() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input03.txt"#).expect("Failed to read file");
    //uncomment to use example code
    // input = "^>v<".to_string();
    //create mutable lists for a unique list of visited locations. 
    let mut visited= HashSet::new();
    let mut double_visited= HashSet::new();

    //use x and y variants for stored location of answer 1 and answer 2 (santa & robot) 
    let (mut x,mut y) = (0,0);
    let (mut santa_x,mut santa_y) = (0,0);
    let (mut robot_x,mut robot_y) = (0,0);
    
    //insert initial house (0,0) to all sets
    visited.insert((0, 0));
    double_visited.insert((0, 0));

    // let mut count = 0;
    for (count, character) in input.chars().enumerate() {
        //change house based on character
        //if even move santa, for answer 2, else move robot
        match character {
            '^' => {
                y -= 1;
                if count%2==0 {santa_y-=1;}
                else {robot_y-=1;}
            },
            'v' => {
                y += 1;
                if count%2==0 {santa_y+=1;}
                else {robot_y+=1;}
            },
            '<' => {
                x -= 1;
                if count%2==0 {santa_x-=1;}
                else {robot_x-=1;}
            },
            '>' => {
                x += 1;
                if count%2==0 {santa_x+=1;}
                else {robot_x+=1;}
            },
            _ => {},
        }
        //insert visited values for answer1
        visited.insert((x,y));

        //insert where santa and robot have been for answer2
        //you use the same list, since it asks how many houses have had presents, and so if they each hit the same house, it doesn't stack.
        double_visited.insert((santa_x,santa_y));
        double_visited.insert((robot_x,robot_y));
        

    }


    //get length of lists for answer. 
    let answer1 = visited.len();
    let answer2 = double_visited.len();
    
    //print answers.
   println!("Answer 1 = {answer1}");
   println!("Answer 2 = {answer2}");
    // 4161 too high
}
