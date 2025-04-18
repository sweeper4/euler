use num::{BigInt, One};

pub fn solve() {
    println!("The sum of the digits of 100! is {}", sum_digits_of(factorial(100)));
}

fn sum_digits_of(num: BigInt) -> u32 {
    return num.to_u32_digits().1.iter().sum();
}

fn factorial(n:i32) -> BigInt {
    let mut product:BigInt = BigInt::one();
    let mut i:i32 = 2;
    while i <= n {
        product = product * i;
        i += 1;
    }
    return product;
}