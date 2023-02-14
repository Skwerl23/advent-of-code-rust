
extern crate num_bigint;

use num_bigint::BigInt;

pub fn pon() {
    println!("Running Persistence of number");

    //initialize all used variables
    let one: BigInt = BigInt::from(1);
    let zero: BigInt = BigInt::from(0);
    let ten: BigInt = BigInt::from(10);
    let mut x: BigInt = BigInt::from(1);
    let mut ans:BigInt;
    let mut throttle:BigInt;
    let mut maxsteps = 0;
    let mut stringnum: String;
    let mut count:usize;
    let mut charcount:usize;
    let mut lengthofstring:usize;
    let mut chars: Vec<char>;
    loop {
        if &x%&ten == zero {
            stringnum = x.to_string();
            chars = stringnum.chars().collect();
            charcount = 0;
            lengthofstring = stringnum.len();
            let mut c1:char=chars[0];
            for c2 in chars[1..].iter() {
                charcount+=1;
                if c1 > *c2 {
                    stringnum = stringnum[..charcount].to_string() + &c1.to_string().repeat(lengthofstring - charcount);
                    x=BigInt::parse_bytes(stringnum.as_bytes(), 10).unwrap();
                    break;
                }
                c1=*c2;
            }
        }
        count = 1;
        throttle = x.clone();
        ans=BigInt::from(1);
        while throttle != zero {
            ans*=&throttle % &ten;
            throttle = &throttle/&ten;
            if ans == zero {
                break;
            }
//            println!("temp: {temp} throttle: {throttle} ans:{ans}");
            if throttle == zero {
                if ans >= ten {
                    //don't clone ans as it's being reset immediately anyhow - reduced time by a quarter second.
                    throttle = ans;
                    ans=BigInt::from(1);
                    count+=1
                }
            }
        }


//        println!("{x}");
        if count > maxsteps {
            println!("{x} took {count} steps!");
            maxsteps = count.clone();
        }
//        println!("{x}");
        if count == 11{break;}
        x += &one;
    }
}
