#![allow(dead_code)]
#![allow(unused_variables)]
use num::bigint::Sign;
use num::integer::Roots;
use num::BigInt;
use num::FromPrimitive;

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use num_traits::One;
use num_traits::Zero;
use primes::PrimeSet;
use sorted_vec::SortedVec;
use num_traits::int::PrimInt;

use crate::fraction::Fraction;

pub fn divisors_include_one_and_n(n:u64) -> HashSet<u64> {
    let mut factors = vec!();
    let prime_factors = prime_factors_of(n.into());
    for (prime, count) in prime_factors {
        for i in 0..count {
            factors.push(prime);
        }
    }
    let mut set = HashSet::new();
    for i in 0..(2_u32.pow((factors.len()).try_into().unwrap())) {
        let mut factor:u64 = 1;
        for j in 0..factors.len() {
            if (i % (2_u32.pow((j + 1) as u32))) / 2_u32.pow(j as u32) > 0 {
                factor = factor * factors[j];
            }
        }
        set.insert(factor);
    }
    return set;
}

pub fn divisors_include_one(n:u64) -> HashSet<u64> {
    let mut factors = vec!();
    let prime_factors = prime_factors_of(n.into());
    for (prime, count) in prime_factors {
        for i in 0..count {
            factors.push(prime);
        }
    }
    let mut set = HashSet::new();
    for i in 0..(2_u32.pow((factors.len()).try_into().unwrap()) - 1) {
        let mut factor:u64 = 1;
        for j in 0..factors.len() {
            if (i % (2_u32.pow((j + 1) as u32))) / 2_u32.pow(j as u32) > 0 {
                factor = factor * factors[j];
            }
        }
        set.insert(factor);
    }
    return set;
}

pub struct Decimal {
    pub int_part: i32,
    pub finite_decimal_part: Vec<u32>,
    pub repeating_decimal_part: Vec<u32>
}

impl Decimal {
    pub fn show(self) -> String {
        return format!("{}.{}({})",
                self.int_part.to_string(),
                self.finite_decimal_part.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x.to_string())),
                self.repeating_decimal_part.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x.to_string()))
        );
    }
}

pub fn transform_fraction_into_decimal_notation(mut num: i32, denum: i32) -> Decimal {
    let int_part = num / denum;
    num -= int_part * denum;
    let mut division_steps:Vec<u32> = vec![];
    let mut former_nums = vec![];
    while num > 0 && !former_nums.contains(&num) {
        former_nums.push(num);
        num *= 10;
        division_steps.push((num / denum).try_into().unwrap());
        num -= (num / denum) * denum;
    }
    if num == 0 {
        return Decimal{
            int_part: int_part,
            finite_decimal_part: division_steps,
            repeating_decimal_part: vec![]
        }
    } else {
        let mut former_num_iter = former_nums.iter();
        let mut former_num = former_num_iter.next();
        let mut division_steps_iter = division_steps.iter();
        let mut division_step = division_steps_iter.next();
        let mut finite_decimal_part:Vec<u32> = vec![];
        let mut repeating_decimal_part:Vec<u32> = vec![];
        while former_num.is_some() && *former_num.unwrap() != num {
            finite_decimal_part.push(*division_step.unwrap());
            division_step = division_steps_iter.next();
            former_num = former_num_iter.next();
        }
        while division_step.is_some() {
            repeating_decimal_part.push(*division_step.unwrap());
            division_step = division_steps_iter.next();
        }
        return Decimal{
            int_part: int_part,
            finite_decimal_part: finite_decimal_part,
            repeating_decimal_part: repeating_decimal_part
        }
    }
}

pub fn prime_factors_of(mut n:u64) -> HashMap<u64,u64> {
    let mut primes = primes::Sieve::new();
    let mut prime_factors = HashMap::new();
    for prime in primes.iter() {
        let mut count = 0;
        while n > 1 && n % prime == 0 {
            count += 1;
            n /= prime;
        }
        if count > 0 {
            prime_factors.insert(prime, count);
        }
        if n == 1 {
            break;
        }
    }
    return prime_factors;
}

pub fn prime_factors_of_efficient(original_n:u64, primes: &mut primes::Sieve, memo: &mut HashMap<u64, HashMap<u64,u64>>) -> HashMap<u64,u64> {
    let mut n = original_n;
    if memo.contains_key(&n) {
        return memo.get(&n).unwrap().clone();
    }
    let mut prime_factors = HashMap::new();
    for prime in primes.iter() {
        let mut count = 0;
        while n > 1 && n % prime == 0 {
            count += 1;
            n /= prime;
            if memo.contains_key(&n) {
                prime_factors.insert(prime, count);
                let memoized_factors = memo.get(&n).unwrap().clone();
                let mut final_factors = HashMap::new();
                for (key,value) in prime_factors.iter() {
                    if !final_factors.contains_key(key) {
                        final_factors.insert(*key, 0);
                    }
                    final_factors.insert(*key, final_factors.get(key).unwrap() + *value);
                }
                for (key,value) in memoized_factors.iter() {
                    if !final_factors.contains_key(key) {
                        final_factors.insert(*key, 0);
                    }
                    final_factors.insert(*key, final_factors.get(key).unwrap() + *value);
                }
                memo.insert(original_n, final_factors.clone());
                return final_factors;
            }
        }
        if count > 0 {
            prime_factors.insert(prime, count);
        }
        if n == 1 {
            break;
        }
    }
    memo.insert(original_n, prime_factors.clone());
    return prime_factors;
}

pub fn totient(n: u64, primes: &mut primes::Sieve, prime_factors_memo: &mut HashMap<u64, HashMap<u64, u64>>) -> u64 {
    let mut count = 1;
    let factors = prime_factors_of_efficient(n, primes, prime_factors_memo);
    for (a,b) in factors {
        count *= a.pow(b as u32) - a.pow(b as u32 - 1);
    }
    return count;
}

pub fn number_of_divisors(n: u64) -> u64 {
    let factors = prime_factors_of(n);
    let mut divisor_count = 1;
    for (factor, count) in factors {
        divisor_count *= count+1;
    }
    return divisor_count;
}

pub fn permute<T: Clone + Copy + Eq + std::hash::Hash>(list:Vec<T>) -> Vec<Vec<T>> {
    let mut final_result = Vec::new();
    if list.is_empty() {
        final_result.push(vec![]);
        return final_result;
    }
    let first = list[0];
    let recursive_result:Vec<Vec<T>> = permute::<T>(list[1..list.len()].to_vec());
    for result in recursive_result {
        for i in 0..result.len()+1 {
            let mut mut_result:Vec<T> = result.clone();
            mut_result.insert(i,first);
            final_result.push(mut_result);
        }
    }
    return final_result;
}

pub fn is_pandigital(mut n:u64, m:u64) -> bool {
    let mut digits = HashSet::new();
    while n > 0 {
        digits.insert(n % 10);
        n /= 10;
    }
    if digits.len() != m.try_into().unwrap() {
        return false;
    }
    for i in 1..(m+1) {
        if !digits.contains(&i) {
            return false;
        }
    }
    return true;
}

pub fn a_choose_b<N: PrimInt>(a:N, b:N) -> N {
    let one = N::one();
    let mut numerators = {
        let mut list = vec![];
        let mut i = b+one;
        while i < a+one {
            list.push(i);
            i = i + one;
        }
        list
    };
    let mut denominators = {
        let mut list = vec![];
        let mut i = one;
        while i <= a-b {
            list.push(i);
            i = i + one;
        }
        list
    };
    for i in 0..numerators.len() {
        if numerators[i] == one {
            continue;
        }
        for j in 0..denominators.len() {
            let common = gcd(&numerators[i], &denominators[j]);
            if common > one {
                numerators[i] = numerators[i] / common;
                denominators[j] = denominators[j] / common;
            }
        }
    }
    let mut result = one;
    for num in numerators {
        result = result * num;
    }
    for denum in denominators {
        result = result / denum;
    }
    return result;
}

pub fn prime_sieve(n:u64) -> SortedVec<u64> {
    let mut primes:SortedVec<u64> = SortedVec::new();
    if n >= 2 {
        primes.insert(2);
    }
    if n >= 3 {
        primes.insert(3);
    }
    if n >= 5 {
        primes.insert(5);
    }
    let mut i = 2;
    loop {
        for j in &[5, 9, 11, 15, 17, 21, 27, 29, 35, 39, 41, 45, 47, 51, 57, 59] { //all primes (except 2,3,5) are one of these offsets (+2) from a multiple of 60
            if i + j > n {
                return primes;
            }
            let mut is_prime = true;
            for prime in primes.iter() {
                if (i + j) % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes.insert(i + j);
            }
        }
        i += 60;
    }
}

pub fn gcd<N: One + Zero + PartialOrd + Rem<Output = N> + Clone>(a:&N, b:&N) -> N {
    let a = (*a).clone();
    let b = (*b).clone();
    if b <= N::one() {
        return N::one();
    } else if a < b {
        return gcd(&b,&a);
    } else if a.clone() % b.clone() == N::zero() {
        return b;
    } else {
        let new_a = b.clone();
        let new_b = a % b;
        return gcd(&new_a, &new_b);
    }
}

pub fn is_prime(n:u64) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i <= n / i {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

pub fn is_triangular(n: u64) -> bool {
    let solution = (((1 + 8*n) as f64).sqrt() as u64 - 1)/2;
    return solution * (solution + 1) / 2 == n;
}

pub fn is_permute(a: &u64, b: &u64, radix: usize) -> bool {
    let mut a = *a;
    let mut b = *b;
    let mut digit_counts = vec![];
    for i in 0..radix {
        digit_counts.push(0);
    }
    while a > 0 {
        digit_counts[a as usize % radix] += 1;
        a /= radix as u64;
    }
    while b > 0 {
        digit_counts[b as usize % radix] -= 1;
        b /= radix as u64;
    }
    for i in 0..digit_counts.len() {
        if digit_counts[i] != 0 {
            return false;
        }
    }
    return true;
}

pub fn num_length<N: Div<Output = N> + PartialOrd + Zero + One + Copy>(n: N, radix: N) -> N {
    let mut n = n;
    let mut count = N::zero();
    while n > N::zero() {
        count = count + N::one();
        n = n / radix;
    }
    return count;
}

pub fn continued_fraction_of_sqrt(num: u64) -> (u64, Vec<u64>) {
    let int_part = num.sqrt();
    let mut numerator = 1;
    let mut denominator_modifier = int_part;
    let mut repeating_part = vec![];
    let non_repeating_part = int_part;
    let mut points_encountered = vec![];
    loop {
        let numerator_modifier = denominator_modifier;
        let denominator = (num - numerator_modifier * numerator_modifier) / numerator;
        let new_int_part = (((num as f64).sqrt() + numerator_modifier as f64) / denominator as f64).floor() as u64;
        denominator_modifier = denominator * new_int_part - numerator_modifier;
        numerator = denominator;
        let new_point = (new_int_part, denominator_modifier, numerator);
        if points_encountered.contains(&new_point) {
            break;
        }
        points_encountered.push(new_point);
        repeating_part.push(new_int_part);
    }
    return (non_repeating_part, repeating_part);
}

pub fn calculate_partial_sum(int_part: BigInt, repeating_part: Vec<u64>, term_count: usize) -> Fraction<BigInt> {
    let mut fraction = Fraction::new(BigInt::new(Sign::Plus, vec![0]),BigInt::from_u64(1).unwrap()).unwrap();
    let mut i = term_count - 1;
    loop {
        if i == 0 {
            break;
        }
        i -= 1;
        fraction = fraction + Fraction::new(BigInt::new(Sign::Plus, vec![repeating_part[i % repeating_part.len()] as u32]), BigInt::new(Sign::Plus, vec![1])).unwrap();
        fraction = fraction.inverse();
    }
    fraction = fraction + Fraction::new(int_part, BigInt::new(Sign::Plus, vec![1])).unwrap();
    return fraction;
}

pub fn power_set<N: Clone + Eq + Hash>(initial: HashSet<N>) -> Vec<Vec<N>> {
    let mut set_of_sets = vec![vec![]];
    for val in initial {
        let mut new_set_of_sets = vec![];
        for set in set_of_sets {
            let mut new_set = set.clone();
            new_set.push(val.clone());
            new_set_of_sets.push(new_set);
            new_set_of_sets.push(set);
        }
        set_of_sets = new_set_of_sets;
    }
    return set_of_sets;
}

pub fn exponentiation_under_mod<N: Hash + Copy + Ord + Eq + Zero + One + Mul + Add + Div<Output = N> + Rem<Output = N>>(mut base: N, mut power: N, modulus: N, memo: &mut HashMap<(N,N,N),N>) -> N {
    if memo.contains_key(&(base,power,modulus)) {
        return *memo.get(&(base,power,modulus)).unwrap();
    }
    let mut answer = N::one();
    let two = N::one() + N::one();
    base = base % modulus;
    if base == N::zero() {
        return N::zero();
    }
    while power > N::zero() {
        if power % two == N::one() {
            answer = answer * base % modulus;
        }
        power = power / two;
        answer = answer * answer % modulus;
    }
    memo.insert((base,power,modulus), answer);
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_include_one_and_n_works() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(5);
        assert_eq!(super::divisors_include_one_and_n(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        set.insert(8);
        assert_eq!(super::divisors_include_one_and_n(8), set);
    }

    #[test]
    fn divisors_include_one_works() {
        let mut set = HashSet::new();
        set.insert(1);
        assert_eq!(super::divisors_include_one(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        assert_eq!(super::divisors_include_one(8), set);
    }

    #[test]
    fn permute_trivial_vec_works() {
        let mut expected = vec![];
        expected.push(vec![]);
        assert_eq!(super::permute::<u64>(vec![]),expected);
    }

    #[test]
    fn permute_simple_vec_works() {
        let mut expected = vec![];
        expected.push(vec![1]);
        assert_eq!(super::permute::<u64>(vec![1]),expected);
        expected.clear();
        expected.push(vec![1,2,3]);
        expected.push(vec![1,3,2]);
        expected.push(vec![2,1,3]);
        expected.push(vec![2,3,1]);
        expected.push(vec![3,1,2]);
        expected.push(vec![3,2,1]);
        let actual = super::permute::<u64>(vec![1, 2, 3]);
        assert_eq!(actual.len(), expected.len());
        for i in 0..actual.len() {
            assert!(expected.contains(&actual[i]));
        }
    }

    #[test]
    fn gcd_works() {
        assert_eq!(9, super::gcd(&27, &18));
        assert_eq!(800, super::gcd(&38729600,&800));
        assert_eq!(1, super::gcd(&1,&100));
        assert_eq!(1, super::gcd(&100,&1));
    }

    #[test]
    fn prime_sieve_works() {
        let mut primes:SortedVec<u64> = SortedVec::new();
        primes.insert(2);
        assert_eq!(primes, super::prime_sieve(2));
        primes.insert(3);
        assert_eq!(primes, super::prime_sieve(3));
        primes.insert(5);
        assert_eq!(primes, super::prime_sieve(6));
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
        assert_eq!(super::prime_sieve(92), primes);
    }

    #[test]
    fn continued_fraction_of_sqrtt() {
        let (a,b) = continued_fraction_of_sqrt(2);
        assert_eq!(a,1);
        assert_eq!(b, vec![2]);
        let (a,b) = continued_fraction_of_sqrt(3);
        assert_eq!(a,1);
        assert_eq!(b, vec![1,2]);
        let (a,b) = continued_fraction_of_sqrt(5);
        assert_eq!(a,2);
        assert_eq!(b, vec![4]);
    }

    #[test]
    fn calculate_partial_sum_works() {
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 1);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![1]), BigInt::new(Sign::Plus, vec![1])).unwrap());
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 2);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![3]), BigInt::new(Sign::Plus, vec![2])).unwrap());
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 3);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![7]), BigInt::new(Sign::Plus, vec![5])).unwrap());
    }

    #[test]
    fn prime_factors_of_efficient_works() {
        let mut primes = primes::Sieve::new();
        let mut memo = HashMap::new();
        let factors = prime_factors_of_efficient(5, &mut primes, &mut memo);
        assert_eq!(factors.keys().len(), 1);
        assert_eq!(*factors.get(&5).unwrap(), 1);
        let factors = prime_factors_of_efficient(25, &mut primes, &mut memo);
        assert_eq!(factors.keys().len(), 1);
        assert_eq!(*factors.get(&5).unwrap(), 2);
    }

    #[test]
    fn power_set_workd() {
        let mut a = vec![];
        a.push(1);
        a.push(2);
        a.push(3);
        let power = power_set(a);
        assert_eq!(power.len(), 8);
        assert!(power.contains(&HashSet::new()));
        assert!(power.contains(&vec![1]));
        assert!(power.contains(&vec![2]));
        assert!(power.contains(&vec![3]));
        assert!(power.contains(&vec![1,2]));
        assert!(power.contains(&vec![1,3]));
        assert!(power.contains(&vec![3,2]));
        assert!(power.contains(&vec![1,2,3]));
    }
}
