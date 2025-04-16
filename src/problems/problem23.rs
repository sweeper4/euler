use number_theory;
use std::collections::BTreeSet;

pub fn solve() {
    println!("The sum of all non-abundant numbers is {}", find_sum_of_non_abundant_sums_under(28123));
}

fn find_sum_of_non_abundant_sums_under(n:u64) -> u64 {
    let abundant_nums = find_all_abundant_nums_under(n);

    let mut i = 1;
    let mut sum = 0;
    while i < n {
        let mut abundant_iter = abundant_nums.iter();
        let mut abundant_num = abundant_iter.next();
        let mut abundant = false;
        while abundant_num.is_some() && *(abundant_num.unwrap()) < i {
            if abundant_nums.contains(&(i - abundant_num.unwrap())) {
                abundant = true;
                break;
            }
            abundant_num = abundant_iter.next();
        }
        if !abundant {
            // println!("{} is not the sum of two abundant numbers", i);
            sum += i;
        } else {
            // println!("{} is the sum of {} and {}", i, abundant_num.unwrap(), i - abundant_num.unwrap());
        }
        i += 1;
    }
    return sum;
}

fn find_all_abundant_nums_under(n:u64) -> BTreeSet<u64> {
    let mut abundant_nums = BTreeSet::new();
    let mut i = 12;
    while i <= n {
        let divisors = number_theory::number_theory::divisors_include_one(i);
        let sum:u64 = divisors.iter().sum();
        if sum > i {
            abundant_nums.insert(i);
        }
        i += 1;
    }
    return abundant_nums;
}