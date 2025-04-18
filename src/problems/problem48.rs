use num::{BigInt, Zero};
use std::convert::TryInto;

pub fn solve() {
    let mut sum = BigInt::zero();
    for i in 1..1000 {
        sum += BigInt::new(num::bigint::Sign::Plus,vec![i]).pow(i.try_into().unwrap());
    }
    println!("{}", sum);
}