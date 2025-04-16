use number_theory::number_theory;

pub fn solve() {
    // let primes = number_theory::prime_sieve(99);
    let primes = number_theory::prime_sieve(999999);
    let mut families = vec![];
    for i in 0..primes.len() {
        let prime = primes[i];
        for family_of_prime in get_all_families_of_prime(prime) {
            if families.contains(&family_of_prime) {
                continue;
            }
            let mut prime_count = 0;
            for j in 0..10 {
                if j == 0 && family_of_prime[0] == 10 {
                    continue;
                }
                let new_prime = substitue_into_family(&family_of_prime, j);
                if primes.contains(&new_prime) {
                    prime_count += 1;
                }
            }
            families.push(family_of_prime);
            if prime_count == 8 {
                println!("{}", primes[i]);
                return;
            }
        }
    }
    println!("Ya mussed up");
}

fn get_all_families_of_prime(mut prime: u64) -> Vec<Vec<u64>> {
    let mut digit_array = vec![];
    while prime >= 1 {
        let remainder = prime % 10;
        digit_array.push(remainder);
        prime /= 10;
    }
    let mut modified_char_arrays = vec![];
    for i in 1..(2_usize.pow(digit_array.len() as u32))-1 {
        let mut modified_char_array = vec![];
        for j in 0..digit_array.len() {
            let digit = if (i / 2_usize.pow(j as u32)) % 2 == 0 {
                digit_array[j]
            } else {
                10
            };
            modified_char_array.push(digit);
        }
        modified_char_array.reverse();
        if modified_char_array[modified_char_array.len()-1] == 10 {
            continue;
        }
        modified_char_arrays.push(modified_char_array);
    }
    return modified_char_arrays;
}

fn substitue_into_family(family: &Vec<u64>, val: u64) -> u64 {
    let mut result = 0;
    for i in family {
        result *= 10;
        if *i == 10 {
            result += val;
        } else {
            result += *i;
        }
    }
    return result;
}
