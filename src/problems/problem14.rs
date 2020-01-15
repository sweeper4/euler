use std::collections::HashMap;

pub fn problem14() {
    println!("The longest collatz sequence beginning with a value less than 1,000,000 begins with {}", get_longest_collatz_under(1000000));
}

fn get_longest_collatz_under(n: u64) -> u64 {
    let mut memoized: HashMap<u64,u64> = HashMap::new();
    let mut i = 1;
    let mut max_length = 0;
    let mut max_index = 0;
    while i < n {
        let length = get_collatz_len(i, &mut memoized);
        if length > max_length {
            max_length = length;
            max_index = i;
        }
        memoized.insert(i,length);
        i += 1;
    }
    return max_index;
}

fn get_collatz_len(n: u64, memoized: &mut HashMap<u64, u64>) -> u64 {
    if n == 1 {
        return 1;
    }
    match (memoized).get(&n) {
        Some(value) => *value,
        None => if n % 2 == 0 {
                let result = get_collatz_len(n / 2, memoized);
                (memoized).insert(n, result + 1);
                return result + 1;
            } else {
                let result = get_collatz_len(n * 3 + 1, memoized);
                (memoized).insert(n, result + 1);
                return result + 1;
            }
    }
}