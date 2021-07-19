use std::convert::TryInto;
use std::collections::HashSet;

pub fn problem44() {
    println!("The pentagonal values a and b, where a+b and a-b are also pentagonal, minimizing abs(a-b), has a difference of {}", find_least_pentagonal_difference());
}

fn find_least_pentagonal_difference() -> u64 {
    let mut pentagonal_nums = HashSet::new();
    let mut largest_generated_pentagonal_number = 0;
    let mut diff_index = 0;
    let mut diff = 1;
    loop {
        let mut lesser_index = 0;
        let mut lesser = 1;
        loop {
            let greater = diff + lesser;
            while greater >= largest_generated_pentagonal_number {
                largest_generated_pentagonal_number = get_nth_pentagonal_number(pentagonal_nums.len().try_into().unwrap());
                pentagonal_nums.insert(largest_generated_pentagonal_number);
            }
            if pentagonal_nums.contains(&greater) {
                let sum = lesser + greater;
                while sum >= largest_generated_pentagonal_number {
                    largest_generated_pentagonal_number = get_nth_pentagonal_number(pentagonal_nums.len().try_into().unwrap());
                    pentagonal_nums.insert(largest_generated_pentagonal_number);
                }
                if pentagonal_nums.contains(&sum) {
                    return diff;
                }
            }
            lesser_index += 1;
            while lesser_index+5 >= pentagonal_nums.len() {
                largest_generated_pentagonal_number = get_nth_pentagonal_number(pentagonal_nums.len().try_into().unwrap());
                pentagonal_nums.insert(largest_generated_pentagonal_number);
            }
            lesser = get_nth_pentagonal_number(lesser_index.try_into().unwrap());
            if lesser + diff < get_nth_pentagonal_number((lesser_index + 1).try_into().unwrap()) {
                break;
            }
        }
        diff_index += 1;
        while diff_index+5 >= pentagonal_nums.len() {
            largest_generated_pentagonal_number = get_nth_pentagonal_number(pentagonal_nums.len().try_into().unwrap());
            pentagonal_nums.insert(largest_generated_pentagonal_number);
        }
        diff = get_nth_pentagonal_number(diff_index.try_into().unwrap());
    }
}

fn get_nth_pentagonal_number(n:u64) -> u64 {
    return (n+1) * (3 * (n+1) - 1) / 2;
}