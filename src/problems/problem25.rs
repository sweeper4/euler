use num::{BigInt, One};

pub fn solve() {
    let mut a = BigInt::one();
    let mut b = BigInt::one();
    let mut b_index = 2;

    while b.to_str_radix(10).len() < 1000 {
        let c = a.clone();
        a = b;
        b = a.clone() + c;
        b_index += 1;
    }

    println!("The first fibonacci value with more than 1000 characters occurs at index {}", b_index);
}