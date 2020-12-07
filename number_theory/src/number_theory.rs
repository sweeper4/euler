#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use sorted_vec::SortedVec;

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
    let mut primes = primes::PrimeSet::new();
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

pub fn permute<T: Clone + Copy + Eq + std::hash::Hash>(list:Vec<T>) -> HashSet<Vec<T>> {
    let mut final_result = HashSet::new();
    if list.is_empty() {
        final_result.insert(vec![]);
        return final_result;
    }
    let first = list[0];
    let recursive_result:HashSet<Vec<T>> = permute::<T>(list[1..list.len()].to_vec());
    for result in recursive_result {
        for i in 0..result.len()+1 {
            let mut mut_result:Vec<T> = result.clone();
            mut_result.insert(i,first);
            final_result.insert(mut_result);
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

pub fn a_choose_b(a:u128, b:u128) -> u128 {
    //This could be made better by dividing when possible, to reduce overflows
    return ((b+1)..(a+1)).fold(1, |a,b| a * b)/(1..b+1).fold(1, |a,b| a * b);
}

pub fn prime_sieve(n:u64) -> SortedVec<u64> {
    let mut primes:SortedVec<u64> = SortedVec::new();
    primes.insert(2);
    primes.insert(3);
    primes.insert(5);
    let mut i = 6;
    loop {
        let mut prime = true;
        for j in primes.iter() {
            if j * j > i + 1 {
                break;
            }
            if (i + 1) % j == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.insert(i + 1);
        }
        prime = true;
        for j in primes.iter() {
            if j * j > i + 5 {
                break;
            }
            if (i + 5) % j == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.insert(i + 5);
        }
        i += 6;
        if i >= n {
            break;
        }
    }
    return primes;
}

pub fn gcd(a:u64, b:u64) -> u64 {
    if b <= 1 {
        return 1;
    }
    if a < b {
        return gcd(b,a);
    }
    if a % b == 0 {
        return b;
    }
    return gcd(b, a % b);
}