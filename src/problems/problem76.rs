use std::collections::HashMap;

pub fn solve() {
    let mut memo = HashMap::new();
    let count = count_of_sums(100, &mut memo, 100);
    println!("{}", count);
}

fn count_of_sums(total: u64, memo: &mut HashMap<u64, u64>, limit: u64) -> u64 {
    if memo.contains_key(&total) {
        return *memo.get(&total).unwrap();
    }
    let mut count = 0;
    for i in 1..limit+1 {
        if i > total {
            break;
        }
        count += count_of_sums(total - i, memo, i) + 1;
    }
    memo.insert(total, count);
    return count;
}

fn count_of_sums_2(total: u64, limit: u64) -> Vec<u64> {
    let mut count = 0;
    for i in 1..limit+1 {
        if i > total {
            break;
        }
        count += count_of_sums_2(total - i, i) + 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::problems::problem76::count_of_sums_2;

    use super::{count_of_sums, solve};

    #[test]
    fn test() {
        solve();
    }

    #[test]
    fn test_count_of_sums() {
        let mut memo = HashMap::new();
        assert_eq!(0, count_of_sums_2(1, 1));
        assert_eq!(1, count_of_sums_2(2, 2));
        assert_eq!(2, count_of_sums_2(3, 3));
        assert_eq!(4, count_of_sums_2(4, 4));
        assert_eq!(6, count_of_sums_2(5, 5));
        assert_eq!(6, count_of_sums_2(6, 6));
    }
}