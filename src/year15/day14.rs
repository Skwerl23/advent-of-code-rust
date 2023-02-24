use std::fs::read_to_string;
use std::collections::HashMap;

pub fn day14() {

    let input  = read_to_string(r#"c:\tools\adventofcode\2015\input14.txt"#).expect("Failed to read file.");
    let test_input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_string();
    //curiously this gets set once, but it wants it to be mutable
    let test_answer = solve_day14(test_input, 1000);
    //get both test_answers, for validation
    if test_answer == (1120, 689) {
        //explain passing, and run final test
        println!("Test case passed. Running actual input...");
        let answer = solve_day14(input, 2503);
        println!("Answer1 = {}", answer.0);
        println!("Answer2 = {}", answer.1);
    } 
    else {
        //explain and display failures.
        println!("Test case failed, try again.");
        println!("Test Answer1 = {}", test_answer.0);
        println!("Test Answer2 = {}", test_answer.1);
        
    }
}

fn solve_day14(input: String, timelimit: i32) -> (i32, i32) {
    let split_input: Vec<&str> = input.lines().collect();
    //each half is completely different and works differently, thus 2 different functions

    //this function loops for each second, and determines the reindeers distance, and the winner
    //need a racing hashmap and point hashmap... 
    let mut race_hashmap:HashMap<String, (i32, i32, bool)> = HashMap::new();
    let mut point_hashmap:HashMap<String, i32> = HashMap::new();
    let mut racers_stats: Vec<(String, i32, i32, i32)> = Vec::new();
    //by moving every parsing step into this loop, we reduced processing time from 13ms to 2ms - particularly racers_stats
    for line in &split_input {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let name: String = split_line[0].to_string();
        let speed: i32 = split_line[3].parse().unwrap();
        let run_time: i32 = split_line[6].parse().unwrap();
        let rest_time: i32 = split_line[13].parse().unwrap();
        race_hashmap.insert(name.clone(), (0, 0, true));
        point_hashmap.insert(name.clone(), 0);
        racers_stats.push((name, speed, run_time, rest_time));
    }
    for _ in 0..timelimit {
        let mut winner: i32 = 0;
        // let mut winner_name: String = String::new();
        for line in &racers_stats {
            //we could probably create an array of tuples of this info, but yea.
            let (name, speed, run_time, rest_time) = line;
            let (current_distance, current_time, moving) = race_hashmap.get_mut(name).unwrap();
            if *moving {
                *current_distance += *speed;
                *current_time += 1;
                if *current_time == *run_time {
                    *current_time = 0;
                    *moving = false;
                }
            }
            else {
                *current_time += 1;
                if *current_time == *rest_time {
                    *current_time = 0;
                    *moving = true;
                }

            }
            // race_hashmap[&name] = (current_distance, current_time, moving);
            if *current_distance > winner {
                winner = *current_distance;
            }
        } 
        //dereference and modify winner with an additional point.
        //could have used the original hashmap, but i decided to split their scores for simplicity.
        for (k, v) in &race_hashmap {

            let (distance,_,_) = *v;
            if distance == winner {
                *point_hashmap.get_mut(k).unwrap() += 1;
            }

        }

    }
    //initialize answers
    let mut answer1 = 0;
    let mut answer2 = 0;
    for (k, v) in point_hashmap {
        //split race_hashmap from key and get winner distance for part 1
        let (distance,_,_) = race_hashmap[&k];
        answer1 = i32::max(distance, answer1);
        //check point value for part 2 and find winner distance
        answer2 = i32::max(v, answer2);
    }
    (answer1, answer2)   
}
