//requres md5 vs 0.7.0 crate.
//this day takes 37 seconds on my laptop, seems way too slow. it does 10 million hashes. so thats 3 million per second or so. 
//i'm curious if the formatting to string and so on is part of the problem.
//using --release reduced it to 9 seconds
//ok it was reduced to 3 seconds by just comapring hte bytes. first 5 is 0,0, <= 0f
//and first 6 is 0,0,0 (hex 000000 is 3 0s in a row)
pub fn day04() {
    //import data file
    let input = "iwrupvqb".to_string();
    //test data should result in 609043
    // let input = "abcdef".to_string();
    
    let mut iteration: i32 = 0;

    loop {
        // concat string and iterator
        let value: String = format!("{input}{iteration}");
        // create digest of value
        let hash: md5::Digest = md5::compute(value) ;
        //validate hash, if true, print answer and exit loop
        if hash.as_ref()[0..2] == [0, 0] && hash.as_ref()[2] <= 0x0f {
            println!("Answer 1 = {iteration}");
            break;
        }
        iteration+=1;
    }

    //separate loop for part 2 to speed up the code about 10%, less if checks
    loop {
        // concat string and iterator
        let value: String = format!("{input}{iteration}");
        // create digest of value
        let hash: md5::Digest = md5::compute(value) ;
        //validate hash, if true, print answer and exit loop
        if hash.as_ref()[0..3] == [0, 0, 0] {
            println!("Answer 2 = {iteration}");
            break;
        }
        iteration+=1;
    }


}
