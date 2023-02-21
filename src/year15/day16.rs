use std::fs::read_to_string;
use std::collections::HashMap;

struct AuntStats {
children: i32,
cats: i32,
samoyeds: i32,
pomeranians: i32,
akitas: i32,
vizslas: i32,
goldfish: i32,
trees: i32,
cars: i32,
perfumes: i32,
}

pub fn day16() {
    let input  = read_to_string(r#"c:\tools\adventofcode\2015\input16.txt"#).expect("Failed to read file.");
    let split_input = input.split_terminator('\n');
    let mut aunt_hashmap: HashMap<usize, AuntStats> = HashMap::new();
    for i in 1..=input.matches('\n').count() {
        aunt_hashmap.insert(i, AuntStats { children: -1, cats: -1, samoyeds: -1, pomeranians: -1, akitas: -1, vizslas: -1, goldfish: -1, trees: -1, cars: -1, perfumes: -1 });
    }
    for line in split_input {
        let split_line: Vec<&str> = line.split_whitespace().collect(); 
        let aunt_key:usize = split_line[1].trim_end_matches(':').parse().unwrap();
        // let current_aunt = aunt_hashmap.get_mut(&aunt_key);
        for i in 0..3 {
            let stat_a_name: &str = split_line[i*2+2].trim_end_matches(':');
            let stat_a_value: i32 = split_line[i*2+3].trim_end_matches(',').parse().unwrap();
            if let Some(current_aunt) = aunt_hashmap.get_mut(&aunt_key) {
                match stat_a_name {
                    "children" => current_aunt.children = stat_a_value,
                    "cats" => current_aunt.cats = stat_a_value,
                    "samoyeds" => current_aunt.samoyeds = stat_a_value,
                    "pomeranians" => current_aunt.pomeranians = stat_a_value,
                    "akitas" => current_aunt.akitas = stat_a_value,
                    "vizslas" => current_aunt.vizslas = stat_a_value,
                    "goldfish" => current_aunt.goldfish = stat_a_value,
                    "trees" => current_aunt.trees = stat_a_value,
                    "cars" => current_aunt.cars = stat_a_value,
                    "perfumes" => current_aunt.perfumes = stat_a_value,
                    _ => {} 
                }
            }
        }
    }
    let mut answer1:usize = 0;
    let mut answer2:usize = 0;
    for i in 1..=input.matches('\n').count() {
        
        let (match_a, match_b) = compare_fields(&aunt_hashmap[&i]);

        if match_a == 3 {
            answer1 = i;
        }
        else if match_b == 3  {
            answer2 = i;
        }
    }
    println!("Answer 1 = {}", answer1);
    println!("Answer 2 = {}", answer2);


}


fn compare_fields(comparer: &AuntStats) -> (i32, i32) {
    let correct_aunt = AuntStats {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };
    let mut matches_a = 0;
    if comparer.children == correct_aunt.children {matches_a+=1}
    if comparer.cats == correct_aunt.cats {matches_a+=1}
    if comparer.samoyeds == correct_aunt.samoyeds {matches_a+=1}
    if comparer.pomeranians == correct_aunt.pomeranians {matches_a+=1}
    if comparer.akitas == correct_aunt.akitas {matches_a+=1}
    if comparer.vizslas == correct_aunt.vizslas {matches_a+=1}
    if comparer.goldfish == correct_aunt.goldfish {matches_a+=1}
    if comparer.trees == correct_aunt.trees {matches_a+=1}
    if comparer.cars == correct_aunt.cars {matches_a+=1}
    if comparer.perfumes == correct_aunt.perfumes {matches_a+=1}

    let mut matches_b = 0;
    //not only do we have to use > < operators, but we have to know the value, so...
    // pomeranians and goldfish have to be >= 0 (meaning we know it's value) to be able to match it.
    // -1 is considered unknown, zero could be known.
    if comparer.children == correct_aunt.children {matches_b+=1}
    if comparer.cats > correct_aunt.cats {matches_b+=1}
    if comparer.samoyeds == correct_aunt.samoyeds {matches_b+=1}
    if comparer.pomeranians < correct_aunt.pomeranians && comparer.pomeranians >= 0 {matches_b+=1}
    if comparer.akitas == correct_aunt.akitas {matches_b+=1}
    if comparer.vizslas == correct_aunt.vizslas {matches_b+=1}
    if comparer.goldfish < correct_aunt.goldfish && comparer.goldfish >= 0 {matches_b+=1}
    if comparer.trees > correct_aunt.trees {matches_b+=1}
    if comparer.cars == correct_aunt.cars {matches_b+=1}
    if comparer.perfumes == correct_aunt.perfumes {matches_b+=1}    
    (matches_a, matches_b)
}
