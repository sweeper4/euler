use number_theory::number_theory;
use std::collections::BTreeMap;

pub fn solve() {
    let primes:Vec<u64> = number_theory::prime_sieve(10000).iter().map(|x| return *x).filter(|x| *x > 999).collect();
    let mut primes_map: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for prime in primes {
        let hash = hash_function(prime);
        primes_map.entry(hash).or_insert(vec![]).push(prime);
    }
    for (_key, value) in primes_map {
        if value.len() < 3 {
            continue;
        }
        for i in 0..value.len() - 1 {
            for j in (i+1)..value.len() {
                if value.contains(&(2 * value[j] - value[i])) && value[i] != 1487 {
                    println!("Found {}, {}, {}", value[i], value[j], 2 * value[j] - value[i]);
                }
            }
        }
    }
}

fn hash_function(mut n: u64) -> u64 {
    let mut values:Vec<u64> = vec![0;10];
    while n > 0 {
        values[n as usize % 10] += 1;
        n /= 10;
    }
    let mut lowest = 0;
    for i in 0..values.len() {
        for _ in 0..values[i] {
            lowest = lowest * 10 + i;
        }
    }
    return lowest as u64;
}