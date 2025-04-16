use big_int::big_int;
use std::convert::TryInto;

pub fn solve() {
    let mut sum = big_int::BigInt::new(0);
    for i in 1..1000 {
        sum += big_int::BigInt::new(i).pow(i.try_into().unwrap());
    }
    println!("{}", sum.show_formatted());
}