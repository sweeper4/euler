use std::collections::{HashMap, HashSet};

use number_theory::number_theory::{exponentiation_under_mod, power_set};

pub fn solve() {
    let mut exponentiation_memo = HashMap::new();
    let limit = 1234567891;
    let n = 100_000_000_000_000;
    let total_sets = exponentiation_under_mod(2, n, limit, &mut exponentiation_memo);
    let mut total = 0;
    let mut logs = vec![];
    for _ in 0..n {
        logs.push(0);
    }
    for i in 1..n+1 {
        let num_sets_with_i = total_sets / 2;
        let num_sets_with_i_but_no_multiples = exponentiation_under_mod(2, n - i, limit, &mut exponentiation_memo);
        let sum = sum_all_ints_between(n / (i + 1) + 1, n / i);
        let num_sets_with_i_elevisor = num_sets_with_i - num_sets_with_i_but_no_multiples;
        total = (total + num_sets_with_i_elevisor * sum) % limit;
        for i in n/(i+1)+1..n/i+1 {
            logs[(i-1) as usize] = num_sets_with_i_elevisor;
        }
    }
    for i in 1..n+1 {
        println!("{}:{}",i,logs[(i-1) as usize]);
    }
    println!("{}",total);
}

fn brute_force_experiment(n: u64) -> u64 {
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
    let mut total = 0;
    for i in 0..n {
        println!("{}:{}", i+1, num_results[i as usize]);
        total += (i+1) * num_results[i as usize];
    }
    return total;
}

fn clever_thing(n: u64) -> u64 {
    let mut exponentiation_memo = HashMap::new();
    let limit = 1234567891;
    let total_sets = exponentiation_under_mod(2, n, limit, &mut exponentiation_memo);
    let mut total = 0;
    for i in 1..n+1 {
        let exp = exponentiation_under_mod(2, n - n/i, limit, &mut exponentiation_memo);
        let val = (total_sets / 2 + limit - exp) % limit;
        println!("{}:{}", i, val);
        total += i * val;
    }
    return total;
}

fn cleverer_thing(n: u64) -> u64 {
    let mut exponentiation_memo = HashMap::new();
    let limit = 1234567891;
    let total_sets = exponentiation_under_mod(2, n, limit, &mut exponentiation_memo);
    let mut total = 0;
    // let mut logs = vec![];
    // for _ in 0..n {
    //     logs.push(0);
    // }
    for i in 1..n+1 {
        let num_sets_with_i = (total_sets / 2) % limit;
        let num_sets_with_i_but_no_multiples = exponentiation_under_mod(2, n - i, limit, &mut exponentiation_memo);
        let sum = sum_all_ints_between(n / (i + 1) + 1, n / i) % limit;
        let num_sets_with_i_elevisor = (limit + num_sets_with_i - num_sets_with_i_but_no_multiples) % limit;
        total = (total + num_sets_with_i_elevisor * sum) % limit;
        // for i in n/(i+1)+1..n/i+1 {
        //     logs[(i-1) as usize] = num_sets_with_i_elevisor;
        // }
    }
    // for i in 1..n+1 {
    //     println!("{}:{}",i,logs[(i-1) as usize]);
    // }
    // println!("{}",total);
    return total;
}

fn sum_all_ints_between(start: u64, end: u64) -> u64 {
    return end * (end + 1) / 2 - start * (start - 1) / 2;
}

#[cfg(test)]
mod tests {
    use crate::problems::problem944::cleverer_thing;

    use super::{brute_force_experiment, clever_thing, solve};

    #[test]
    fn test() {
        assert_eq!(brute_force_experiment(10), 4927);
        assert_eq!(clever_thing(10), 4927);
        assert_eq!(cleverer_thing(10), 4927);
    }

    #[test]
    fn test_solve() {
        cleverer_thing(1_000_000_00);
    }
}
