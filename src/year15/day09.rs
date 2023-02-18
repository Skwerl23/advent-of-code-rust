use std::fs::read_to_string;
use std::collections::HashSet;

pub fn day09() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input09.txt"#).expect("Failed to read file");
    let test_input_1 = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141".to_string();
    let answer:(i32, i32);
    let test_answer: (i32, i32) = solve_day09(test_input_1);
    if test_answer == (605, 982) {
        println!("Test case solved correctly. The solutions are...");
        answer = solve_day09(input);
        println!("Answer1 = {}", answer.0);
        println!("Answer2 = {}", answer.1);
    }
    else {
        println!("Test case failed to solve. Try again.");
        println!("Test Answer1 = {}", test_answer.0);
        println!("Test Answer2 = {}", test_answer.1);

    }

}

fn solve_day09(input: String) -> (i32, i32) {
    let input_lines = input.split_terminator('\n');
    let mut distance_vectors:Vec<(&str, &str, i32)> = Vec::new();

    let mut destinations: HashSet<&str> = HashSet::new();
    for line in input_lines {
        let data_vec: Vec<&str> = line.split(' ').collect();
        destinations.insert(data_vec[2]);
        destinations.insert(data_vec[0]);
        let left = data_vec[0];
        let right = data_vec[2];
        let dist: i32 = data_vec[4].parse::<i32>().unwrap();    
        distance_vectors.push((left, right, dist));
        distance_vectors.push((right, left, dist));
    }
    let mut seen: HashSet<&str> = HashSet::new();
    let mut min_travel_distance: i32 = i32::max_value();
    let mut max_travel_distance: i32 = 0;
    for destination in destinations.iter() {
        seen.insert(&destination);
        find_shortest_path(destination, &mut seen, 0, &mut min_travel_distance, &mut max_travel_distance, &destinations, &distance_vectors);
        seen.remove(destination);
    }
    // answer = TSP();
    (min_travel_distance, max_travel_distance)
}

fn find_shortest_path<'a> (current_location: &str, mut seen: &mut HashSet<&'a str>, current_travel_distance: i32, min_travel_distance: &mut i32, max_travel_distance: &mut i32, destinations: &HashSet<&str>, distance_vectors: &Vec<(&str, &'a str, i32)>)  {
    if &seen.len() == &destinations.len() {
        // println!("{:?},{}", seen, current_travel_distance);
        if current_travel_distance < *min_travel_distance {
            *min_travel_distance = current_travel_distance;
        }
        if current_travel_distance > *max_travel_distance {
            *max_travel_distance = current_travel_distance;
        }
        
    }
    for route in distance_vectors {
        let (left, right, distance) = &route;
        
        if left == &current_location {
            if !seen.contains(right) {
                seen.insert(right.clone());
                find_shortest_path(right, &mut seen, current_travel_distance+distance, min_travel_distance, max_travel_distance, &destinations, &distance_vectors);
                seen.remove(right.clone());
            }
        }
    }
}