use std::fs::read_to_string;

pub fn day23() {
    // get input data (almost pointless, but it makes it universal. with user data, not test data.)
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input23.txt"#).expect("Failed to read file.");
    let instructions: Vec<&str> = input.lines().collect();

    // get answers for part 1 and 2
    let answer1 = solve_day23(&instructions, 0);
    let answer2 = solve_day23(&instructions, 1);

    // print answers
    println!("Answer 1 = {}", answer1);
    println!("Answer 2 = {}", answer2);
    

}

fn solve_day23(instructions: &Vec<&str>, start: usize ) -> usize {
    // not sure if this solves every option... my input only has a and b
    // if only a and b exists this works. (or any 2 values, just convert below)
    // a starts with zero or 1 for part 1 and 2
    let mut a: usize = start;
    let mut b: usize = 0;
    let mut i: usize = 0;
    // loop until broken (out of bounds reached)
    loop {
        if i >= instructions.len() {
            // break if out of bounds
            break;
        }
        // line gets set to line split on spaces
        let line: Vec<&str> = instructions[i].split_whitespace().collect();

        // jump if one (not odd)
        if line[0] == "jio" {
            let number: &str = &line[1].trim_end_matches(',');
            let jump: &str = &line[2][..1];
            let value: usize = line[2][1..].parse::<usize>().unwrap();
            // rule is "a" equals one and number is "a" not b
            if number == "a" && a == 1 {
                if jump == "+" {
                    i += value;
                }
                else {
                    i -= value;
                }
                continue;
            }
        }
        // jump if even
        else if line[0] == "jie" {
            let number: &str = &line[1].trim_end_matches(',');
            let jump: &str = &line[2][..1];
            let value: usize = line[2][1..].parse::<usize>().unwrap();
            // rule is "a" equals one and number is "a" not b
            if number == "a" && a%2 == 0 {
                if jump == "+" {
                    i += value;
                }
                else {
                    i -= value;
                }
                continue;
            }
        }
        // this is an always rule - if jmp then jump
        else if line[0] == "jmp" {
            let number: usize = line[1][1..].parse::<usize>().unwrap();
            let jump: &str = &line[1][..1];
            // determine if positive or negative
            if jump == "+" {
                i += number;
            }
            else {
                i -= number;
            }
            continue;
        }
        // logic for incrementing a/b
        else if line[0] == "inc" {
            if line[1] == "a" {
                a+=1;
            }
            else {
                b+=1;
            }
        }
        // logic for tripling a/b
        else if line[0] == "tpl" {
            if line[1] == "a" {
                a*=3;
            }
            else {
                b*=3;
            }
        }
        // logic for halfing a/b
        else if line[0] == "hlf" {
            if line[1] == "a" {
                a/=2;
            }
            else {
                b/=2;
            }
        }
        // if reached, increase i
        i+=1;
    }
    // return answer
    b
}