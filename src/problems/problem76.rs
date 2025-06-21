use std::collections::HashMap;

pub fn solve() {
    let mut memo = HashMap::new();
    let count = count_of_all_sums(100, 100, &mut memo) - 1;
    println!("{}", count);
}

fn count_of_all_sums(total: u64, limit: u64, memo: &mut HashMap<(u64, u64), usize>) -> usize {
    if memo.contains_key(&(total, limit)) {
        return *memo.get(&(total, limit)).unwrap();
    }
    if total == 0 {
        return 1;
    }
    let mut results = 0;
    for i in 1..limit+1 {
        if i > total {
            break;
        }
        let sub_result = count_of_all_sums(total - i, i, memo);
        results += sub_result;
    }
    memo.insert((total, limit), results);
    return results;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{count_of_all_sums, solve};

    #[test]
    fn test() {
        solve();
    }

    #[test]
    fn test_count_of_sums() {
        let mut memo = HashMap::new();
        for i in 1..10 {
            println!("{:?}", count_of_all_sums(i, i, &mut memo));
        }
        assert_eq!(1, count_of_all_sums(1, 1, &mut memo));
        assert_eq!(2, count_of_all_sums(2, 2, &mut memo));
        assert_eq!(3, count_of_all_sums(3, 3, &mut memo));
        assert_eq!(5, count_of_all_sums(4, 4, &mut memo));
        assert_eq!(7, count_of_all_sums(5, 5, &mut memo));
        assert_eq!(11, count_of_all_sums(6, 6, &mut memo));
    }
}