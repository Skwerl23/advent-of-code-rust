use std::fs::read_to_string;
use serde_json::Value;

pub fn day12() {
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input12.txt"#).expect("Failed to read file");
    let test_input = r#"[1,{"c":"reds","q":{"a":"red", "g":7},"b":2},3]"#.to_string();
    //curiously this gets set once, but it wants it to be mutable


    let test_answer = solve_day12(test_input);
    //get both test_answers, for validation
    if test_answer == (13,6) {
        //explain passing, and run final test
        println!("Test case passed. Running actual input...");
        let answer = solve_day12(input);
        println!("Answer1 = {}", answer.0);
        println!("Answer2 = {}", answer.1);

    } 
    else {

        println!("Test case failed, try again.");
        println!("Test Answer1 = {}", test_answer.0);
        println!("Test Answer2 = {}", test_answer.1);
        
    }
}

fn solve_day12(input: String) -> (i64, i64) {
    let json_parsed: Value = serde_json::from_str(&input).unwrap();
    // let json_object = json_parsed.as_object();
    let mut sum: i64 = 0;
    let mut non_red_sum: i64 = 0;
    let mut dont_add_because_red_exists = false;
    iterate_json_object(&json_parsed, &mut sum, &mut non_red_sum, &mut dont_add_because_red_exists);
    // println!("{:?}", json_parsed);
    (sum,non_red_sum)
}



fn iterate_json_object(object: &Value, sum: &mut i64, non_red_sum: &mut i64, dont_add_because_red_exists: &mut bool) {

    match object {
        Value::Number(number) => {
            if *dont_add_because_red_exists {
                *sum+= number.as_i64().unwrap();
            }
            else {
                *non_red_sum+= number.as_i64().unwrap();
                *sum+= number.as_i64().unwrap();
            }
            
        }
        Value::Null | Value::Bool(_) | Value::String(_) => {
            // If the object is a leaf node, just print its value

        }
        Value::Array(array) => {
            // If the object is an array, recursively iterate over its items
            for item in array {
                
                iterate_json_object(item, sum, non_red_sum, dont_add_because_red_exists);
            }
        }
        Value::Object(map) => {
            // If the object is a map, recursively iterate over its values
            let mut parent_node = true;
            if *dont_add_because_red_exists {
                parent_node = false;
            }            
            if let Value::Object(map) = object {
                if map.values().any(|v| v == "red") {
                    *dont_add_because_red_exists = true;
                }
            }            
            for value in map.values() {
                iterate_json_object(value, sum, non_red_sum, dont_add_because_red_exists);
            }

            if parent_node {
                *dont_add_because_red_exists = false;
            }
        }
    }
}