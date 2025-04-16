use std::convert::TryInto;
use std::collections::HashSet;

pub fn solve() {
    println!("The sum of all products p, where a*b=p, where abp is 1-9 pandigital, is {}", sum_products_of_pandigital_equations());
}

fn sum_products_of_pandigital_equations() -> u32 {
    let pandigital_combinations = number_theory::number_theory::permute(vec![1,2,3,4,5,6,7,8,9]);
    let mut valid_products = HashSet::new();
    for pandigital_combination in pandigital_combinations {
        for i in 0..9 {
            for j in i+1..9 {
                if is_pandigital_product(&pandigital_combination, i, j) {
                    valid_products.insert(get_pandigital_product(&pandigital_combination, j));
                }
            }
        }
    }
    return valid_products.iter().fold(0, |acc, x| acc + x);
}

fn is_pandigital_product(pandigital_combination:&Vec<u32>, i:u32, j:u32) -> bool {
    let mut a = 0;
    let mut b = 0;
    let mut product = 0;
    for (index, value) in pandigital_combination.iter().enumerate() {
        if index <= i.try_into().unwrap() {
            a = a * 10 + value;
        } else if index <= j.try_into().unwrap() {
            b = b * 10 + value;
        } else {
            product = product * 10 + value;
        }
    }
    return a * b == product;
}

fn get_pandigital_product(pandigital_combination:&Vec<u32>, j:u32) -> u32 {
    let mut product = 0;
    for (index, value) in pandigital_combination.iter().enumerate() {
        if index > j.try_into().unwrap() {
            product = product * 10 + value;
        }
    }
    return product;
}