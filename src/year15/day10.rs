
pub fn day10() {
    let mut input = "3113322113".to_string();
    // the way our loop works, we add a zero at the end so we don't cause a "index out of bounds" panic
    const ITERATIONS:i32 = 51; //they said 40 iterations, but it needs 41? 0 to 40 run which is 41 iterations. / 51 for part 2. 
    let mut it_count = 0;
    //curiously this gets set once, but it wants it to be mutable
    let mut answer1: usize = 0;
    let mut answer2: usize = 0;
    while it_count < ITERATIONS {
        let numbers: Vec<char> = input.chars().collect();
        let mut count = 1;
        input = "".to_string();
        for i in 0..numbers.len() {
            //if we're on the last digit, we can't compare to nothing after. or we get an index out of bounds
            //the first if check fails, so it doesn't try the second one. and runs the else statement.
            if i != numbers.len()-1 && numbers[i] == numbers[i+1]{
                count+=1;
            }
            else {
                input+=format!("{}{}",count, numbers[i]).as_str();
                count = 1;
            }

        }
        // add a zero to prevent out of bounds panic.
        if it_count == 40 {
            answer1 = numbers.len();
        }
        if it_count == 50 {
            answer2 = numbers.len();
            break;
        }

        it_count += 1;
    }
    // 252594 is too small
    // 329356 is the right answer? 
   println!("Answer 1 = {answer1}");
   println!("Answer 2 = {answer2}");

}
