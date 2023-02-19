pub fn day11() {
    let input = "hxbxwxba".to_string();
    let test_input = "ghijklmn".to_string();
    //curiously this gets set once, but it wants it to be mutable
    let test_answer = solve_day11(test_input);
    //get both test_answers, for validation
    if test_answer == ("ghjaabcc".to_string(), "ghjbbcdd".to_string()) {
        //explain passing, and run final test
        println!("Test case passed. Running actual input...");
        let answer = solve_day11(input);
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


fn solve_day11(input: String) -> (String, String) {
    //this function increments string letter by letter.
    let mut chars: Vec<char> = input.chars().collect();

    // let mut count = 0;
    let char_len = &chars.len();
    let mut run = 0;
    let mut answer1: Vec<char> = Vec::new();
    let mut answer2: Vec<char> = Vec::new();
    while run < 2 {
        if valid_chars(&chars) {
            if run == 0 {
                answer1 = chars.clone();
            } else {
                answer2 = chars.clone();
            }
            run += 1;
        }
        // count+=1;
        chars[char_len-1] = (chars[char_len-1] as u8 + 1) as char;
        for i in 1..chars.len() {
            // println!("current char is {}",chars[i]);
            if chars[i] == '{' {
                // println!("This happened");
                chars[i] = 'a';
                chars[i-1] = (chars[i-1] as u8 + 1) as char;
            }

        }
        // println!("{:?}", chars);
    }
    (answer1.into_iter().collect(), answer2.into_iter().collect())


}


fn valid_chars(chars: &Vec<char>)-> bool {
    //this function checks if the string of characters is a vlid string.
    //first check is if it has a incrementing 3 characters in a row.
    let mut three_increment = false;

    for i in 0..chars.len()-3 {
        if chars[i] as u8 + 1 == chars[i+1] as u8  && chars[i] as u8 + 2 == chars[i+2] as u8 {
            three_increment = true;
            break
        }
    }

    //second check validates if there are 2 pairs.
    //basically if the next character is equal to this character we have one pair, go ahead and skip the next loop
    //if we find at least one more pair our count is 2+
    //2+ = good
    let mut skip = false;
    let mut double_match_count = 0;
    for i in 0..chars.len()-1 {
        if skip {
            skip = false;
            continue;
        }
        if chars[i] == chars[i+1] {
            double_match_count+=1;
            skip = true;
        }
    }

    //final rule is that it can't contain i o or l due to confuse. (I O or L)
    let mut bad_chars: bool = true;
    for i in ['i','o','l'] {
        if chars.contains(&i) {
            bad_chars = false;
        }
    }

    //if all 3 are true, set answer to true. and finally return answer.
    let answer: bool;
    if three_increment && bad_chars && double_match_count > 1{
        answer = true;
    }
    else {
        answer = false;
    }
    answer
}