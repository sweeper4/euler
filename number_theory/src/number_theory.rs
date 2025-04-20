#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ops::Div;
use std::ops::Rem;
use num_traits::One;
use num_traits::Zero;
use primes::PrimeSet;
use sorted_vec::SortedVec;
use num_traits::int::PrimInt;

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
    // x(x+1)/2=n
    // x^2+x=2n
    // x^2+x-2n=0
    // -1+-sqrt(1-4*1*-2n)/2
    let solution = (((1 + 8*n) as f64).sqrt() as u64 - 1)/2;
    return solution * (solution + 1) / 2 == n;
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
}
