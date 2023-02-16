use std::time::Instant;
use std::process::exit;
use std::io::{stdin,stdout,Write};

mod year15;
fn main() {
    //get year from user
    let mut year=String::with_capacity(3);
    print!("What year do you want to run?: (15 has been built)\n");
    let _=stdout().flush();
    stdin().read_line(&mut year).expect("Did not enter a correct string");
    let year = format!("{:0>width$}", year.trim(), width=2);
    //validate year is viable
    match &year[..] { 
        "15"=>{},
        _=>{println!("You entered an invalid Year. Exiting...");exit(0)}
    }
    //get day from user
    let mut day=String::with_capacity(3);
    print!("What day do you want to run?: (day 1 through 7 have been solved)\n");
    let _=stdout().flush();
    stdin().read_line(&mut day).expect("Did not enter a correct string");
    let day = format!("{:0>width$}", day.trim(), width=2);
    //combine day and year into single value
    let answer=format!("{year}.{day}");
    println!("Processing {answer}");
    //start a timer for run time of code
    let now = Instant::now();
    //validate day/year combination and run if possible
    match &answer[..] { 
        "15.01"=>{year15::day01::day01();},
        "15.02"=>{year15::day02::day02();},
        "15.03"=>{year15::day03::day03();},
        "15.04"=>{year15::day04::day04();},
        "15.05"=>{year15::day05::day05();},
        "15.06"=>{year15::day06::day06();},
        "15.07"=>{year15::day07::day07();},
        "15.08"=>{year15::day08::day08();},
        "15.09"=>{year15::day09::day09();},
        "15.10"=>{year15::day10::day10();},
        _=>{println!("You entered an invalid Day. Exiting...");exit(0)}
    }
    //finalize timer and get elapsed time in seconds.
    let elapsed = now.elapsed();
    println!("Total Run Time: {:.2?}", elapsed);
}
