use std::collections::{HashMap, HashSet};

use number_theory::number_theory::{exponentiation_under_mod, power_set};

pub fn solve() {
    let mut total = 0;
    let mut exponentiation_memo = HashMap::new();
    let limit = 1234567891;
    let n = 100_000_000_u64;
    let total_sets = exponentiation_under_mod(2, n, limit, &mut exponentiation_memo);
    for i in 1..n+1 {
        total = (total + total_sets / 2 + limit - exponentiation_under_mod(2, n - n/i, limit, &mut exponentiation_memo)) % limit;
        if i % 1_000 == 0 {
            println!("{}", i);
        }
    }
    print!("{}", total);
}

fn brute_force_experiment(n: u64) {
    let mut set = HashSet::new();
    let mut num_results = vec![];
    for i in 1..n+1 {
        set.insert(i);
        num_results.push(0);
    }
    let vecs = power_set(set);
    for vec in vecs {
        for i in 0..vec.len() {
            let a = vec[i];
            for j in 0..vec.len() {
                if i == j {
                    continue;
                }
                let b = vec[j];
                if b % a == 0 {
                    num_results[(a-1) as usize] += 1;
                    break;
                }
            }
        }
    }
    for i in 0..n {
        println!("{}:{}", i+1, num_results[i as usize]);
    }
}

fn clever_thing(n: u64) {
    let mut exponentiation_memo = HashMap::new();
    let limit = 1234567891;
    let total_sets = exponentiation_under_mod(2, n, limit, &mut exponentiation_memo);
    for i in 1..n+1 {
        let val = (total_sets / 2 + limit - exponentiation_under_mod(2, n - n/i, limit, &mut exponentiation_memo)) % limit;
        println!("{}:{}", i, val);
    }
}

#[cfg(test)]
mod tests {
    use super::{brute_force_experiment, clever_thing, solve};

    #[test]
    fn test() {
        brute_force_experiment(10);
        clever_thing(10);
    }

    #[test]
    fn test_solve() {
        solve();
    }
}
