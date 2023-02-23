use std::{fs::read_to_string, cmp::{max, min}};

pub fn day21() {
    // get input data (almost pointless, but it makes it universal. with user data, not test data.)
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input21.txt"#).expect("Failed to read file.");
    let boss: Vec<&str> = input.split_terminator('\n').collect();

    // get answers for part 1 and 2
    let (answer1, answer2)= recursive_fight(&boss);
    
    println!("Answer 1 = {}", answer1);
    println!("Answer 2 = {}", answer2);
    

}



fn recursive_fight(boss: &Vec<&str>) -> (i32, i32){
    let mut answer1 = i32::MAX;
    let mut answer2 = 0;
    //all the data from the webpage hard coded in
    let shop_weapons: Vec<(i32,i32,i32)> = vec!(
        (8,4,0),
        (10,5,0),
        (25,6,0),
        (40,7,0),
        (74,8,0));
    let shop_armors: Vec<(i32,i32,i32)> = vec!(
        //add 0,0,0 since armor is optional
        (0,0,0),
        (13,0,1),
        (31,0,2),
        (53,0,3),
        (75,0,4),
        (102,0,5));
    let shop_rings: Vec<(i32,i32,i32)> = vec!(
        //added 0,0,0 since armor is optional
        (0,0,0),
        (25,1,0),
        (50,2,0),
        (100,3,0),
        (20,0,1),
        (40,0,2),
        (80,0,3));

        // instead of recursion and calculating what item i'm in, just do 4 loops
        for (costa, attacka, defensea) in &shop_weapons {
            for (costb, attackb, defenseb) in &shop_armors {
                for (costc, attackc, defensec) in &shop_rings {
                    for (costd, attackd, defensed) in &shop_rings {
                        // rules state you can't buy 2 of the same ring, so skip if they are the same
                        // this wouldn't work if 2 rings had same stats - you'd need to add a identifier
                        if (attackc, defensec) == (attackd, defensed) {continue;}
                        let cost = costa+costb+costc+costd;
                        let attack = attacka+attackb+attackc+attackd;
                        let defense = defensea+defenseb+defensec+defensed;
                        // if i win the fight we can check minimum cost
                        if battle_boss((attack, defense), &boss) {
                            answer1 = min(answer1, cost);
                        }
                        // if we lose the fight, check maximum cost
                        else {
                            answer2 = max(answer2, cost);
                        }


                    }
                }
            }
        }
    (answer1, answer2)
}


fn battle_boss(loadout: (i32, i32), boss: &Vec<&str>) -> bool{
    // initialize values
    let mut boss_hp: i32 = boss[0].split_whitespace().last().unwrap().parse().unwrap();
    let boss_dmg: i32 = boss[1].split_whitespace().last().unwrap().parse().unwrap();
    let boss_def: i32 = boss[2].split_whitespace().last().unwrap().parse().unwrap();
    let mut user_hp = 100;
    let (user_dmg, user_def) = loadout;

    // keep fighting until boss or user is dead
    while boss_hp >0 && user_hp > 0 {
        // simple modify boss first (you always get first hit)
        boss_hp -= max(user_dmg - boss_def, 1);
        // if boss didn't die, take a hit
        if boss_hp > 0 {
            user_hp -= max(boss_dmg - user_def, 1);
        }
    }
    // did boss win or you? return answer.
    if boss_hp > 0 {
        false
    }   else {
        true
    }


}