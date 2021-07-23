use number_theory::number_theory;
use std::collections::HashSet;
use std::time::Instant;

pub fn problem51() {
    // println!("The smallest prime that belongs to an 8 group is {}", find_smallest_n_group_prime(8));
    println!("The smallest prime that belongs to an 8 group is {}", find_smallest_n_group_prime_2(8));
}

// fn find_smallest_n_group_prime(_group:u64) -> u64 {
//     let time1 = Instant::now();
//     let primes = number_theory::prime_sieve_btree_2(1000000);
//     let time2 = Instant::now();

//     println!("generated primes in {:?}", time2 - time1);
//     let mut prime_families:HashSet<String> = HashSet::new();

//     for prime in primes.iter() {
//         for expanded in expand_prime(prime.to_string()) {
//             if !prime_families.contains(&expanded) {
//                 let mut count = 0;
//                 for i in 1..10 {
//                     let subbed_prime = perform_substitution(&expanded, i);
//                     if primes.contains(&subbed_prime) {
//                         count += 1;
//                     }
//                 }
//                 if count == 8 {
//                     for i in 1..10 {
//                         let subbed_prime = perform_substitution(&expanded, i);
//                         if primes.contains(&subbed_prime) {
//                             let time3 = Instant::now();
//                             println!("searched primes in {:?}", time3 - time2);
//                             return subbed_prime;
//                         }
//                     }
//                 }
//                 prime_families.insert(expanded);
//             }
//         }
//     }
//     return 42; //should be unreachable
// }

fn find_smallest_n_group_prime_2(_group:u64) -> u64 {
    let time1 = Instant::now();
    let primes = number_theory::prime_sieve_btree_3(1000000);
    let time2 = Instant::now();

    println!("generated primes in {:?}", time2 - time1);
    let mut prime_families:HashSet<String> = HashSet::new();

    for prime in primes.iter() {
        for expanded in expand_prime(prime.to_string()) {
            if !prime_families.contains(&expanded) {
                let mut count = 0;
                for i in 1..10 {
                    let subbed_prime = perform_substitution(&expanded, i);
                    if primes.contains(&subbed_prime) {
                        count += 1;
                    }
                }
                if count == 8 {
                    for i in 1..10 {
                        let subbed_prime = perform_substitution(&expanded, i);
                        if primes.contains(&subbed_prime) {
                            let time3 = Instant::now();
                            println!("searched primes in {:?}", time3 - time2);
                            return subbed_prime;
                        }
                    }
                }
                prime_families.insert(expanded);
            }
        }
    }
    return 42; //should be unreachable
}

fn expand_prime(prime:String) -> Vec<String> {
    let mut modified_primes:Vec<String> = vec!();
    for i in 1..((2i32.pow(prime.len() as u32))/2) {
        let mut modified_prime = String::new();
        for term in left_pad(2*i,prime.len()).chars() {
            if term == '1' {
                modified_prime.push('*');
            } else {
                match prime.chars().nth(modified_prime.len()) {
                    Some(a) => modified_prime.push(a),
                    None => {}
                }
            }
        }
        modified_primes.push(modified_prime);
    }
    return modified_primes;
}

fn left_pad(value:i32, length:usize) -> String {
    let mut acc:String = format!("{:b}",value);

    while acc.len() < length {
        acc = "0".to_string() + &acc;
    }

    return acc;
}

fn perform_substitution(prime:&String, val:u64) -> u64 {
    let mut constructed_value = 0;
    for term in prime.chars() {
        if term == '*' {
            constructed_value = constructed_value * 10 + val;
        } else {
            constructed_value = constructed_value * 10 + term.to_digit(10).unwrap() as u64;
        }
    }
    return constructed_value;
}