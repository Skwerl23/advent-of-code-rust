use std::fs::read_to_string;

pub fn day15() {
    let input  = read_to_string(r#"c:\tools\adventofcode\2015\input15.txt"#).expect("Failed to read file.");
    let test_input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".to_string();
    //curiously this gets set once, but it wants it to be mutable
    let test_answer = solve_day14(test_input);
    //get both test_answers, for validation
    if test_answer == (62842880, 57600000) {
        //explain passing, and run final test
        println!("Test case passed. Running actual input...");
        let answer = solve_day14(input);
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

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn solve_day14(input: String) -> (i32, i32) {
    let split_input = input.lines();
    let mut ingredients: Vec<Ingredient> = Vec::new();
    for i in split_input {
        let parts: Vec<&str> = i.split_whitespace().collect();
        let ingredient: Ingredient = Ingredient {
            capacity:  parts[2].trim_end_matches(',').parse().unwrap(),
            durability:  parts[4].trim_end_matches(',').parse().unwrap(),
            flavor:  parts[6].trim_end_matches(',').parse().unwrap(),
            texture:  parts[8].trim_end_matches(',').parse().unwrap(),
            calories:  parts[10].parse().unwrap(),
        };


        ingredients.push(ingredient);
    }
    let combinations = generate_combinations_sum_100(ingredients.len());
    let mut maximum_score_part_1: i32 = 0;
    let mut maximum_score_part_2: i32 = 0;
    
    for combination in combinations {
        let mut capacity =0;
        let mut durability =0;
        let mut flavor =0;
        let mut texture =0;
        let mut calories =0;
    
        for i in 0..combination.len() {
            capacity+=ingredients[i].capacity * combination[i];
            durability+=ingredients[i].durability * combination[i];
            flavor+=ingredients[i].flavor * combination[i];
            texture+=ingredients[i].texture * combination[i];
            calories+=ingredients[i].calories * combination[i];
        }        
        capacity=i32::max(capacity, 0);
        durability=i32::max(durability, 0);
        flavor=i32::max(flavor, 0);
        texture=i32::max(texture, 0);
        calories=i32::max(calories, 0);

        let currentscore = capacity*durability*flavor*texture;
        // println!("{:?} = {} + ", combination, currentscore);

        maximum_score_part_1 = i32::max(maximum_score_part_1, currentscore);
        if calories == 500 {
            maximum_score_part_2 = i32::max(maximum_score_part_2, currentscore);
        }
    }

    (maximum_score_part_1, maximum_score_part_2)   
}

fn generate_combinations_sum_100(n: usize) -> Vec<Vec<i32>> {

    let mut combinations = vec![];
    let mut current_combination = vec![0; n];

    loop {
        let sum: i32 = current_combination.iter().sum();
        if sum == 100 {
            combinations.push(current_combination.clone());
        }

        if !increment_combination(&mut current_combination) {
            break;
        }
    }

    combinations
}

fn increment_combination(combination: &mut [i32]) -> bool {
    for i in (0..combination.len()).rev() {
        if combination[i] < 100 {
            combination[i] += 1;
            for value in combination.iter_mut().skip(i+1) {
                *value = 0;
            }
            return true;
        }
    }

    false
}

