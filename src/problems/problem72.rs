use std::collections::HashMap;

use number_theory::number_theory::totient;
use primes::Sieve;

pub fn solve() {
    let mut sieve = Sieve::new();
    let mut memo_map = HashMap::new();
    let mut count = 0;
    for i in 2..1_000_001 {
        count += totient(i, &mut sieve, &mut memo_map)
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}