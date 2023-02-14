
extern crate num_bigint;

use num_bigint::BigInt;

pub fn pon2() {
    println!("Running Persistence of number");

    let one: BigInt = BigInt::from(1);
    let zero: BigInt = BigInt::from(0);
    let ten: BigInt = BigInt::from(10);
    let mut x: BigInt = one.clone();
    let mut maxsteps = 0;
    loop {
        let mut throttle: BigInt = x.clone();
        let mut temp: BigInt = &throttle % &ten;
        while temp == zero {
            throttle = &throttle/&ten;
            temp = &throttle % &ten;
        }
        while throttle < x {
            throttle = &throttle * &ten;
            throttle += &temp;
        }
        x=throttle.clone();
        let mut ans=one.clone();
        let mut count=1;
        while throttle != zero {
            temp = &throttle % &ten;
            throttle = &throttle/&ten;
            ans*=temp;
//            println!("temp: {temp} throttle: {throttle} ans:{ans}");
            if throttle == zero {
                if ans >= ten {
                    throttle = ans.clone();
                    ans=one.clone();
                    count+=1
                }
            }
        }
        if count > maxsteps {
            println!("{x} took {count} steps!");
            maxsteps = count.clone();
        }
//        println!("{x}");
        if count == 11{break;}
        x += &one;
    }
}