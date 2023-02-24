use std::fs::read_to_string;
use std::cmp;

pub fn day02() {
    //import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input02.txt"#).expect("Failed to read file");
    let input_vector = input.lines();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for package in input_vector {
        let mut nums = package.split('x').map(|s| s.parse::<i32>().unwrap());
        let (a, b, c) = (nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap());
        let (x, y, z) = (a*b, b*c, a*c);
        let slack = cmp::min(cmp::min(x,y),z);
        answer1 += x*2 + y*2 + z*2 + slack;

        //determine largest value
        let toomuchribbon = cmp::max(cmp::max(a,b),c);
        //2*shortest + 2*medium + 2*longest - 2* longest (this effectively adds the two shortest sides) + 3sides multiplied for bow. 
        let ribbon = (2*a+2*b+2*c-2*toomuchribbon) + (a*b*c);
        answer2 += ribbon;
    }
    println!("Answer1 = {answer1}");
    println!("Answer2 = {answer2}");
}
