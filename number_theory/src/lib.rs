#[allow(unused_imports)]
use std::collections::HashSet;
pub mod number_theory;

#[cfg(test)]
use sorted_vec::SortedVec;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_include_one_and_n_works() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(5);
        assert_eq!(number_theory::divisors_include_one_and_n(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        set.insert(8);
        assert_eq!(number_theory::divisors_include_one_and_n(8), set);
    }

    #[test]
    fn divisors_include_one_works() {
        let mut set = HashSet::new();
        set.insert(1);
        assert_eq!(number_theory::divisors_include_one(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        assert_eq!(number_theory::divisors_include_one(8), set);
    }

    #[test]
    fn permute_trivial_vec_works() {
        let mut expected = HashSet::new();
        expected.insert(vec![]);
        assert_eq!(number_theory::permute::<u64>(vec![]),expected);
    }

    #[test]
    fn permute_simple_vec_works() {
        let mut expected = HashSet::new();
        expected.insert(vec![1]);
        assert_eq!(number_theory::permute::<u64>(vec![1]),expected);
        expected.clear();
        expected.insert(vec![1,2,3]);
        expected.insert(vec![1,3,2]);
        expected.insert(vec![2,1,3]);
        expected.insert(vec![2,3,1]);
        expected.insert(vec![3,1,2]);
        expected.insert(vec![3,2,1]);
        assert_eq!(number_theory::permute::<u64>(vec![1, 2, 3]), expected);
    }

    #[test]
    fn gcd_works() {
        assert_eq!(9, number_theory::gcd(27, 18));
        assert_eq!(800, number_theory::gcd(38729600,800));
        assert_eq!(1, number_theory::gcd(1,100));
        assert_eq!(1, number_theory::gcd(100,1));
    }

    #[test]
    fn prime_sieve_works() {
        let mut primes:SortedVec<u64> = SortedVec::new();
        primes.insert(2);
        assert_eq!(primes, number_theory::prime_sieve(2));
        primes.insert(3);
        assert_eq!(primes, number_theory::prime_sieve(3));
        primes.insert(5);
        assert_eq!(primes, number_theory::prime_sieve(6));
        primes.insert(7);
        primes.insert(11);
        primes.insert(13);
        primes.insert(17);
        primes.insert(19);
        primes.insert(23);
        primes.insert(29);
        primes.insert(31);
        primes.insert(37);
        primes.insert(41);
        primes.insert(43);
        primes.insert(47);
        primes.insert(53);
        primes.insert(59);
        primes.insert(61);
        primes.insert(67);
        primes.insert(71);
        primes.insert(73);
        primes.insert(79);
        primes.insert(83);
        primes.insert(89);
        assert_eq!(number_theory::prime_sieve(92), primes);
    }
}
