use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn day19() {
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input19.txt"#).expect("Failed to read file.");
    let split_input= input.lines();
    let mut molecule: String = "".to_string();
    let mut changes: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in split_input.clone() {
        if line.len() > 50 {
            molecule = line.to_string();
            break;
        }
        if line.matches("=>").count() == 0 {
            continue;
        }
        let change: Vec<&str> = line.split_whitespace().collect();
        if changes.contains_key(change[0]) {
            let key = changes.get_mut(change[0]).unwrap();
            key.push(change[2]);
        } else {
            changes.insert(change[0], vec!(change[2]));
        }
    }

    let answer1 = day_19_solve_part1(&molecule, &changes);
    println!("Answer 1 = {}", answer1);

    //redesign how the changes are made for part 2 - i needed help and the algorithm i found worked differently.
    let mut changes_long: Vec<(String, String)> = Vec::new();
    for line in split_input {
        if line.matches("=>").count() == 0 {
            continue;
        }
        let change: Vec<&str> = line.split_whitespace().collect();
        let key = change[0].to_string();
        let value = change[2].to_string();
        changes_long.push((key, value));
    }

    let answer2 = day_19_solve_part2(&molecule, changes_long);
    println!("Answer 2 = {}", answer2);


}

fn day_19_solve_part1(molecule: &String, changes: &HashMap<&str, Vec<&str>>) -> usize {
    let mut uniquemolecules: HashSet<String> = HashSet::new();
    for i in 0..molecule.len() {
        if changes.contains_key(&molecule[i..i+1]) {
            for v in &changes[&molecule[i..i+1]] {
                uniquemolecules.insert(format!("{}{}{}", &molecule[0..i], v, &molecule[i+1..]));
            }
        }
        if i != molecule.len()-1 && changes.contains_key(&molecule[i..i+2]) {
            for v in &changes[&molecule[i..i+2]] {
                uniquemolecules.insert(format!("{}{}{}", &molecule[0..i], v, &molecule[i+2..]));
            }
        }
    }
    uniquemolecules.len()
}
fn day_19_solve_part2(molecule: &str, changes_long: Vec<(String, String)>) -> i32 {

//so dfs didn't work, it just became astronomical. randomizing the list until you have an e. finds the answer rather fast.
//that being said, if there were 2 ways, and 1 took longer, this would break.
//there is only 1 way, it can be done in any order, but the same "way" - for example...
//converting b to a, c to b, b to a. is the same as c to b, b to a, b to a.
//so most randomized patterns of the solution work.
//sorting the list by longest to shortest breaks my algorithm.
// after a lot of testing, it averages about 4 runs per try. theoretically it could take infinite, but it averages 4
    let mut working_molecule = String::new();
    let mut count:isize=0;
    let mut random_changes = changes_long;
        while working_molecule != "e" {
            count = 0; // reset 
            working_molecule = molecule.to_string(); //reset molecule from changes
            random_changes.shuffle(&mut thread_rng()); //randomize the vector each attempt
            let mut change_applied = true; //set check for loop
            while change_applied {
                change_applied = false; //reset each loop check
                for (key, value) in &random_changes {
                    if working_molecule.matches(value).count() > 0 {
                        working_molecule = working_molecule.replacen(value, key, 1); //
                        count += 1; //increment change count
                        change_applied = true; // keep the loop running
                        break; // this break isn't strictly necessary, but it speeds the process up dramatically.
                    }
                }
            }

        }
    count as i32
}
