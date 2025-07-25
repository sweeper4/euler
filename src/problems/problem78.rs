use std::collections::HashMap;


pub fn solve() { //TODO: Solve
    let mut i = 1;
    let mut memo = HashMap::new();
    loop {
        let count = count_of_all_sums(i, i, &mut memo);
        if count % 1000000 == 0 {
            println!("{}", i);
            return;
        }
        i += 1;
    }
}

fn count_of_all_sums(total: u128, limit: u128, memo: &mut HashMap<(u128, u128), usize>) -> usize {
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
    #[test]
    fn test() {
        super::solve();
    }
}