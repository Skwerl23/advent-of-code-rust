use std::fs::read_to_string;
use std::collections::HashMap;
pub fn day05() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input05.txt"#).expect("Failed to read file");
    // test sample for part 1. should have 2 as answer
    // input = "jchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\nugknbfddgicrmopn\naaa".to_string();
    // test sample for part 2. should give 2 as answer
    // input = "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy".to_string();
    let input_lines = input.split_terminator('\n');
    let vowels = ['a','e','i','o','u'];
    let naughty_pairs = HashMap::from([('a','b'),('c','d'),('p','q'),('x','y')]);
    let mut answer1 = 0;
    let mut answer2 = 0;

    for i in input_lines.clone() {
        let input_chars: Vec<char> = i.to_string().chars().collect();
        let mut vowel_count = 0;
        let mut nice = false;
        for j in 0..input_chars.len()-1 {
            if vowels.contains(&input_chars[j]) {
                vowel_count+=1;
            }
            if input_chars[j] == input_chars[j+1] {
                nice = true;
            }
            //check if naught pair hashmap contains this key. 
            if naughty_pairs.contains_key(&input_chars[j]) {
                //check if the next char matches the naught pair dynamic

                if input_chars[j+1] == naughty_pairs[&input_chars[j]] {
                    nice = false;
                    break;
                }
            }
        }
        //since i compare current to next, i never check last letter if vowel
        //so here we check last letter if vowel.
        if vowels.contains(&input_chars[input_chars.len()-1]) {
            vowel_count += 1;
        }
        if vowel_count <3 {nice = false}
        if nice {
            answer1+=1;
        }
    }
    //part 2
    for i in input_lines.clone() {
        let input_chars: Vec<char> = i.to_string().chars().collect();
        let mut nice = false;
        //create pair finding hashmap
        let mut pairs : HashMap<String, usize> = HashMap::new();
        let mut pair_nice = false;
        //determine if pairs exist
        for j in 0..input_chars.len()-1 {
            //set pair name for hashmap
            let pair_name = format!("{}{}", input_chars[j],input_chars[j+1]);
            
            //if the hashmap has the key, we've found this pair before.
            if pairs.contains_key(&pair_name) {
                //check the current index against first pair found, if they are further apart than 1, we have a winner
                if j - pairs[&pair_name] > 1 {
                    pair_nice = true;
                }
            }
            // otherwise add current name to hashmap
            else {
                pairs.insert(pair_name, j);
            }
        }
        //loop through chars gapped and check for match. xyx zaz etc... if match found, it's "nice"
        for j in 0..input_chars.len()-2 {
            if input_chars[j] == input_chars[j+2] {
                nice = true;
            }
        }
        
        //if nice and a pair of pairs exist
        if nice && pair_nice {
            answer2+=1;
        }
    }


    //print answers.
    println!("Answer 1 = {answer1}");
    println!("Answer 2 = {answer2}");

}
