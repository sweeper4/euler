use big_int::BigInt;

pub fn solve() {
    println!("The sum of the digits of 100! is {}", sum_digits_of(factorial(100)));
}

fn sum_digits_of(num: BigInt) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i < num.num_vec.len() {
        let mut number = num.num_vec[i];
        while number > 0 {
            sum += number % 10;
            number /= 10;
        }
        i += 1;
    }
    return sum;
}

fn factorial(n:i32) -> BigInt {
    let mut product:BigInt = BigInt::new(1);
    let mut i:i32 = 2;
    while i <= n {
        product = product.mul(BigInt::new(i));
        i += 1;
    }
    return product;
}