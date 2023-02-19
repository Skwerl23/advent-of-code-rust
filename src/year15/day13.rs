use std::fs::read_to_string;
use std::collections::HashSet;

//this problem is the same as day 9. the only difference is you have a reverse value as well (people have different happiness traits next to eachother)
pub fn day13() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input13.txt"#).expect("Failed to read file");
    let test_input_1 = "Alice would gain 54 happiness units by sitting next to Bob.\nAlice would lose 79 happiness units by sitting next to Carol.\nAlice would lose 2 happiness units by sitting next to David.\nBob would gain 83 happiness units by sitting next to Alice.\nBob would lose 7 happiness units by sitting next to Carol.\nBob would lose 63 happiness units by sitting next to David.\nCarol would lose 62 happiness units by sitting next to Alice.\nCarol would gain 60 happiness units by sitting next to Bob.\nCarol would gain 55 happiness units by sitting next to David.\nDavid would gain 46 happiness units by sitting next to Alice.\nDavid would lose 7 happiness units by sitting next to Bob.\nDavid would gain 41 happiness units by sitting next to Carol.".to_string();
    let answer:(i32, i32);
    let test_answer: (i32, i32) = solve_day13(test_input_1);
    if test_answer == (330, 286) {
        println!("Test case solved correctly. The solutions are...");
        answer = solve_day13(input);
        println!("Answer1 = {}", answer.0);
        println!("Answer2 = {}", answer.1);
    }
    else {
        println!("Test case failed to solve. Try again.");
        println!("Test Answer1 = {}", test_answer.0);
        println!("Test Answer2 = {}", test_answer.1);

    }

}

fn solve_day13(input: String) -> (i32, i32) {
    let input_lines = input.split_terminator('\n');
    let mut happiness_vectors:Vec<(&str, &str, i32)> = Vec::new();

    let mut people: HashSet<&str> = HashSet::new();
    for line in input_lines {
        let data_vec: Vec<&str> = line.split(' ').collect();
        people.insert(data_vec[0]);
        people.insert(data_vec[10].trim_end_matches('.'));
        let left = data_vec[0];
        let right = data_vec[10].trim_end_matches('.');
        let happiness;
        if data_vec[2] == "gain" {
            happiness = data_vec[3].parse::<i32>().unwrap();
        }
        else {
            happiness = 0 - data_vec[3].parse::<i32>().unwrap();
        }
        

        happiness_vectors.push((left, right, happiness));
        // happiness_vectors.push((right, left, happiness));
    }
    let mut seen: Vec<&str> = Vec::new();
    let mut max_happiness: i32 = 0;
    let mut max_happiness_with_me: i32 = 0;
    for person in &people {
        seen.push(&person);
        find_shortest_path(person, &mut seen, 0,  &mut max_happiness, &mut max_happiness_with_me, &people, &happiness_vectors);
        seen.pop();
    }
    (max_happiness, max_happiness_with_me)
}

fn find_shortest_path<'a> (current_location: &str, mut seen: &mut Vec<&'a str>, current_happiness: i32, max_happiness: &mut i32, max_happiness_with_me: &mut i32, people: &HashSet<&str>, happiness_vectors: &Vec<(&str, &'a str, i32)>)  {
    if &seen.len() == &people.len() {
        let mut final_value: i32= 0;
        //with out me, we find nearest peoples happiness vector.
        for seating_arrangement in happiness_vectors {
            let (left, right, happiness_value) = &seating_arrangement;
            if left == &seen[seen.len()-1] && right == &seen[0]{
                final_value += happiness_value.clone();
            } 
            if right == &seen[seen.len()-1] && left == &seen[0]{
                final_value += happiness_value.clone();
            } 
        }
        
        if current_happiness + final_value > *max_happiness {
            *max_happiness = current_happiness + final_value;
        }
        //with me, we can just not add happiness vector.
        if current_happiness > *max_happiness_with_me {
            *max_happiness_with_me = current_happiness ;
        }
    }
    for seating_arrangement in happiness_vectors {
        let (left, right, happiness_value) = &seating_arrangement;
        let mut backwards_happiness = 0;
        for person_backwards in happiness_vectors {
            let (backwards_left, backwards_right, backwards_happiness_value) = person_backwards;
            
            if backwards_right == left  && backwards_left == right {
                backwards_happiness = *backwards_happiness_value;
            } 
        }

        if left == &current_location {
            if !seen.contains(right) {
                seen.push(right.clone());
                find_shortest_path(right, &mut seen, current_happiness+happiness_value+backwards_happiness, max_happiness, max_happiness_with_me, &people, &happiness_vectors);
                seen.pop();
            }
        }
    }
}
