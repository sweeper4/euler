use std::collections::HashMap;

use number_theory::number_theory::totient;

pub fn solve() {
    let mut highest = 0.0;
    let mut highest_index = 0;
    let mut sieve = primes::Sieve::new();
    let mut prime_factors_memo = HashMap::new();
    for i in 2..1_000_001 {
        let totient = totient(i, &mut sieve, &mut prime_factors_memo);
        let val = i as f64 / totient as f64;
        if highest < val {
            highest = val;
            highest_index = i;
        }
    }
    println!("{}", highest_index);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}