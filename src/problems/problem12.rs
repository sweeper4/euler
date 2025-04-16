use std::collections::HashMap;

pub fn solve() {
    println!("The first triangular number with more than {} divisors is {}", 500, find_first_triangular_number_with_specified_number_of_divisors(500));
}

fn find_first_triangular_number_with_specified_number_of_divisors(divisor_count: u32) -> u32 {
    let mut i: u32 = 0;
    let mut triangular_number: u32 = 0;

    loop {
        i += 1;
        triangular_number += i;

        let count = get_divisor_count(triangular_number);

        if count > divisor_count {
            return triangular_number;
        }
    }
}

fn get_divisor_count(number: u32) -> u32 {
    let mut count = 1;
    let factors: HashMap<u32, u32> = get_prime_factors(number);
    for (_, value) in factors {
        count *= value + 1;
    }
    return count;
}

fn get_prime_factors(n: u32) -> HashMap<u32,u32> {
    let mut new_count: u32 = n;

    let mut factors: HashMap<u32, u32> = HashMap::new();

    let mut i: u32 = 2;

    while new_count > 1 {
        if new_count % i == 0 {
            new_count /= i;
            if factors.contains_key(&i) {
                factors.insert(i, factors.get(&i).unwrap() + 1);
            } else {
                factors.insert(i,1);
            }
        } else {
            i += 1;
        }
    }

    return factors;
}