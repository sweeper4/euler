use num::BigInt;

pub fn solve() {
    println!("The sum of the digits of 2^1000 is {}", BigInt::new(num::bigint::Sign::Plus,vec![2]).pow(1000).to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}