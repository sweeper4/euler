use big_int::BigInt;

pub fn problem16() {
    println!("The sum of the digits of 2^1000 is {}", BigInt::new(2).pow(1000).show().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}