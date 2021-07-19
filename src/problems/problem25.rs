use big_int::BigInt;

pub fn problem25() {
    let mut a = BigInt::new(1);
    let mut b = BigInt::new(1);
    let mut b_index = 2;

    while b.show().len() < 1000 {
        let c = a.clone();
        a = b.clone();
        b = a.clone() + c.clone();
        b_index += 1;
    }

    println!("The first fibonacci value with more than 1000 characters occurs at index {}", b_index);
}