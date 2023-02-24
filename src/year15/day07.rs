use std::fs::read_to_string;
use std::collections::HashMap;

pub fn day07() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input07.txt"#).expect("Failed to read file");
    let input_lines: Vec<&str> = input.split_terminator('\n').collect();

    //if b==0 we don't overwrite it, so solver answers 1 and 2
    let answer1 = calculate_signal(input_lines.clone(), 0);    
    let answer2 = calculate_signal(input_lines.clone(), answer1);
    //print answers.
    // 33706 is wrong
    println!("Answer 1 = {answer1}");
    println!("Answer 2 = {answer2}");

}

fn calculate_signal(input_lines: Vec<&str>, b: i32) -> i32 {
    let mut hash_map:HashMap<&str, i32> = HashMap::new();

    //overwrite b for solution 2
    if b != 0 {
        hash_map.insert("b", b);
    }
    let mut count=0;
    while !hash_map.contains_key("a") {
        let line: &str = input_lines[count];
        //convert line into fields, and determine on/off/toggle
        let items: Vec<&str> = line.split(' ').collect();
    
        //3 example is like: a -> b
        if items.len() == 3 {
            //have to check if key exists, can't overwrite it (aka b)
            if !hash_map.contains_key(items[2]) {
                //check if first item is a number and set hash_map for last item if so
                if let Ok(n1) = items[0].parse::<i32>() {
                    hash_map.insert(items[2], n1);
                }
                //check if hash_map contains first item and set hash_map for last item if so
                else if hash_map.contains_key(items[0]) & !hash_map.contains_key(items[2]){
                    hash_map.insert(items[2], hash_map[items[0]]);
                }
            }
        }
        //4 example is like: NOT a -> b
        if items.len() == 4 {
            //have to check if key exists, can't overwrite it (aka b)
            if !hash_map.contains_key(items[3]) {
               //check if second item is a number and set hash_map for last item to NOT of second item if so
                if let Ok(n1) = items[1].parse::<i32>() {
                    hash_map.insert(items[3], ! n1 & 0xffff);
                }
               //check if second item is in hash_map and set hash_map for last item to NOT of second item if so
               else if hash_map.contains_key(items[1]) & !hash_map.contains_key(items[3]){
                    hash_map.insert(items[3], ! hash_map[items[1]] & 0xffff);
                }
            }
        }
        // 5 example is like: a AND b -> c
        if items.len() == 5 {
            //set bitwise to string item for "logic"
            let bitwise = items[1];
            //since 65535 is max value, set left and right to 70k to start. (indicates they don't exist)
            let mut left: i32 = 70000;
            let mut right: i32 = 70000;

            //check if items positions are integer or in hashmap, and set left and right accordingly
            if let Ok(n1) = items[0].parse::<i32>() {
                left = n1;
            }
            else if hash_map.contains_key(items[0]) {
                left = hash_map[items[0]]
            }
            if let Ok(n2) = items[2].parse::<i32>() {
                right = n2;
            }
            else if hash_map.contains_key(items[2]) {
                right = hash_map[items[2]]
            }

            //if the hashmap doesn't have the key, and right and left are both set perform operations
            if !hash_map.contains_key(items[4]) && right != 70000 && left != 70000 {
                //as stated, perform bitwise operation based on string value
                if bitwise == "AND" {
                    hash_map.insert(items[4], left & right);
                }
                if bitwise == "OR" {
                    hash_map.insert(items[4], left | right);
                
                }
                if bitwise == "LSHIFT" {
                    hash_map.insert(items[4], left << right);
                
                }
                if bitwise == "RSHIFT" {
                    hash_map.insert(items[4], left >> right);
                
                }
            }
        }
        //increment count, and reset if we are done with last item - we must loop all "logic situations" until hashmap key a exists.
        count+=1;
        if count == input_lines.len() {
            count = 0;
        }
    }
    hash_map["a"]
}
