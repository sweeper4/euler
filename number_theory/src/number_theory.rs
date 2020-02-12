#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::convert::TryInto;

pub fn divisors_include_one_and_n(n:u32) -> HashSet<u32> {
    let mut i = 1;
    let mut set = HashSet::new();
    while i <= n {
        if n % i == 0 {
            set.insert(i);
        }
        i += 1;
    }
    return set;
}

pub fn divisors_include_one(n:u32) -> HashSet<u32> {
    let mut i = 1;
    let mut set = HashSet::new();
    while i <= n/2 {
        if n % i == 0 {
            set.insert(i);
        }
        i += 1;
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