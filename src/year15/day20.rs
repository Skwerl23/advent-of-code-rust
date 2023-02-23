
pub fn day20() {
    // initialize values
    let input  = 29000000;
    let mut house = 1;
    let mut answer1 = 0;
    let mut answer2 = 0;
    // loop until we have both answers
    while answer1 == 0 || answer2 == 0 {
        // get both responses
        let (results1, results2) = sum_of_presents(house);
        // if first answer is above input, we have answer 1
        if answer1 == 0 && results1 >= input {
            answer1 = house;
        }
        // if second answer is above input, we have answer 2
        if answer2 == 0 && results2 >= input {
            answer2 = house;
        }
        // increment house number
        house+=1
    }
    // print answers
    println!("answer 1 = {}", answer1);
    println!("answer 2 = {}", answer2);

}

fn sum_of_presents(n: i32) -> (i32, i32) {
    // this just determines all the factors to add
    // initialize values 
    let mut sum = 0;
    let mut sum2 = 0;

    // calculate squares rounded down (perfect squares don't get extra values)
    let d = (n as f64).sqrt().floor() as i32;

    // run up to the square root
    for i in 1..=d {

        // if n is divisible by i, we work with it 
        if n % i == 0 {
            sum += i;

            // part 2 only does first 50 houses
            if  i <= 50 {

                // add the opposite of i, 10 / 2 is 5, 5 is > sqrt of 10 (3 is sqrt floored)
                sum2 += n/i;
            }

            if n/i != i {
                // add the opposite of i, 10 / 2 is 5, 5 is > sqrt of 10 (3 is sqrt floored)

                sum += n / i;

                // part 2 only does first 50 houses
                if n/i <= 50 {
                    sum2 += i;
                }
            }
        }
    }
//return both numbers
(sum * 10, sum2 *11)
}
