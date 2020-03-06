use std::convert::TryInto;

pub fn problem38() {
    println!("The largest 1-9 pandigital created by concatenating 1a, 2a, ... na is {}", find_largest_pandigital());
}

fn find_largest_pandigital() -> u64 {
    let mut max_product = 0;
    let mut a = 1;
    loop {
        let mut product:u64 = 0;
        let mut i = 1;
        while product < 100000000 {
            product = product * 10u64.pow((i * a).to_string().chars().count().try_into().unwrap()) + i * a;
            i += 1;
        }
        
        if product <= 987654321 && number_theory::number_theory::is_pandigital(product, 9) && product > max_product {
            max_product = product;
        }

        if a > 10000 {
            break;
        } else {
            a += 1;
        }
    }

    return max_product;
}