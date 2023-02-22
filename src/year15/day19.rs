use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::rngs::ThreadRng;

pub fn day19() {
    let input  = read_to_string(r#"c:\tools\adventofcode\2015\input19.txt"#).expect("Failed to read file.");
    let split_input = input.split_terminator('\n');
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
        let key = change[0].to_string().chars().rev().collect::<String>();
        let value = change[2].to_string().chars().rev().collect::<String>();
        changes_long.push((key, value));
    }
    molecule = molecule.chars().rev().collect();

    changes_long.reverse();
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
        if i != molecule.len()-1 {
            if changes.contains_key(&molecule[i..i+2]) {
                for v in &changes[&molecule[i..i+2]] {
                    uniquemolecules.insert(format!("{}{}{}", &molecule[0..i], v, &molecule[i+2..]));
                }
            }
        }
    }
    let answer = uniquemolecules.len();
    answer
}
fn day_19_solve_part2(molecule: &String, changes_long: Vec<(String, String)>) -> i32 {
//i'm not sure this works in every scenario, you could put a limit to it releasing your count

//from what i've found, this works if only 1 way is possible.
//and since only 1 way to find e is viable, this works.

    let mut molecule_new = molecule.clone();
    let mut count:isize=0;
    while molecule_new != "e" {
        count = 0;
        let mut molecule_len:isize = molecule_new.len() as isize;
        let mut random_changes;
        let mut rng: ThreadRng;
        molecule_new = molecule.clone();
        rng = thread_rng();
        random_changes=changes_long.clone();
        random_changes.shuffle(&mut rng);

        while molecule_len > 1 {
            let new_changes = random_changes.clone();
            let mut change_applied = false;
            for (value, key) in &new_changes {
                if let Some(index) = molecule_new.find(&*key) {
                    molecule_new.replace_range(index..index+key.len(), &value);
                    count += 1;
                    molecule_len = molecule_new.len() as isize;
                    change_applied = true;
                    break;
                }
            }
            if !change_applied {
                break;
            }
        }

    }

    count as i32
}
