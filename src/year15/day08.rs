use std::fs::read_to_string;

pub fn day08() {
    // import data file
    let input = read_to_string(r#"c:\tools\adventofcode\2015\input08.txt"#).expect("Failed to read file");
    // input = "\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27".to_string();
    let input_lines = input.lines();
    let mut total_bytes = 0;
    let mut line_bytes = 0;
    let mut extended_bytes = 0;
    for i in input_lines {
        total_bytes += i.len();
        line_bytes += i.len()-2;

        if i.contains("\\x") {
            for j in 0..256 {
                //too many repeats so, we check for each hex unicode value
                line_bytes -= i.matches(&format!("\\x{:02x}", j)).count()*3;
            }
        }
        if i.contains("\\\"") {
            line_bytes -= i.matches("\\\"").count();
        }
        if i.contains("\\\\") {
            line_bytes -= i.matches("\\\\").count();
        }
        //since the string could end with \\" we check for this case
        if &i[i.len()-2..i.len()-1] == "\\" {
            line_bytes+=1;
        }

        //solve answer 2 
        //add 2 to line length (for new external quotes)
        extended_bytes += i.len()+2;
        //escape all \'s (count every back slash) 
        extended_bytes+= i.matches('\\').count();
        //escape all "'s (count every quote)
        extended_bytes+= i.matches('\"').count();

    }
    //both answers require the difference value for solution.
    let answer1 = total_bytes - line_bytes;
    let answer2 = extended_bytes - total_bytes;
    //print answers.
    println!("Answer 1 = {answer1}");
    println!("Answer 2 = {answer2}");

}
