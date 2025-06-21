use std::collections::HashMap;

use num_traits::PrimInt;
use primes::{PrimeSet, Sieve};
use std::hash::Hash;

pub fn solve() {
    let mut memo = HashMap::new();
    let mut primes = Sieve::new();
    let mut i = 1_u128;
    loop {
        let result = count_of_all_sums(i, i, &mut memo, &mut primes);
        if result > 5000 {
            println!("{}", i);
            return;
        }
        i += 1;
        if i % 1000 == 0 {
            println!("{}", i);
        }
    }
}

fn count_of_all_sums<N: PrimInt + Hash + std::convert::From<u64>>(total: N, limit: N, memo: &mut HashMap<(N, N), N>, primes: &mut Sieve) -> N {
    if memo.contains_key(&(total, limit)) {
        return *memo.get(&(total, limit)).unwrap();
    }
    if total == N::zero() {
        return N::one();
    }
    let mut results = N::zero();
    let mut prime_index = 0;
    loop {
        let prime: N = primes.get(prime_index).into();
        if prime > total || prime > limit {
            break;
        }
        let sub_result = count_of_all_sums(total - prime, prime, memo, primes);
        results = results + sub_result;
        prime_index += 1;
    }
    memo.insert((total, limit), results);
    return results;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use primes::Sieve;
    use super::count_of_all_sums;

    #[test]
    fn test() {
        super::solve();
    }

    #[test]
    fn count_of_all_sums_test() {
        assert_eq!(5, count_of_all_sums(10_u64, 10, &mut HashMap::new(), &mut Sieve::new()));
    }
}