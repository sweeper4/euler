use std::collections::HashMap;

use number_theory::number_theory::{is_permute, totient};

pub fn solve() {
    let mut min = 1000.0;
    let mut min_index = 0;
    let mut primes = primes::Sieve::new();
    let mut prime_factors_memo = HashMap::new();
    for i in 2..10_000_000 {
        let t = totient(i, &mut primes, &mut prime_factors_memo);
        if is_permute(&i,&t, 10) {
            let val = i as f64 / t as f64;
            if val < min {
                min = val;
                min_index = i;
            }
        }
        if i % 1000 == 0 {
            println!("{}",i);
        }
    }
    println!("{}",min_index);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}