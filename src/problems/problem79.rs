use std::{fs::File, io::{BufReader, BufRead}};
use number_theory::number_theory::permute;

pub fn solve() {
    helper();
}

fn helper() {
    let mut orders = vec![];
    let file = File::open("src/assets/problem79KeyLog.txt").unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line: String = line.unwrap();
        let line = line.as_bytes();
        orders.push(vec![line[0] - '0' as u8, line[1] - '0' as u8, line[2] - '0' as u8]);
    }
    let mut stack: Vec<Vec<u8>> = vec![vec![]];
    while stack.len() > 0 {
        let possible = stack.pop().unwrap();
        let mut passed = true;
        for order in &orders {
            if !satisfies(&possible, order) {
                passed = false;
                let mut new_options = interweave(&possible, order);
                stack.append(&mut new_options);
                stack.sort_by(|a, b| b.len().cmp(&a.len()));
                break;
            }
        }
        if passed {
            println!("{:?}", possible);
            return;
        } else {
            println!("{}", possible.len());
        }
    }
}

fn interweave(a: &Vec<u8>, b: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut list = vec![];
    for _ in 0..a.len() {
        list.push(0);
    }
    for _ in 0..b.len() {
        list.push(1);
    }
    let lists = permute(list);
    let mut results = vec![];
    for list in lists {
        let mut result = vec![];
        let mut a_index = 0;
        let mut b_index = 0;
        for element in list {
            if element == 0 {
                result.push(a[a_index]);
                a_index += 1;
            } else {
                result.push(b[b_index]);
                b_index += 1;
            }
        }
        let a = dedupe(&result);
        if !results.contains(&a) {
            results.push(dedupe(&result));
        }
    }
    return results;
}

fn dedupe(a: &Vec<u8>) -> Vec<u8> {
    let mut b = vec![];
    for i in 0..a.len() {
        if i == 0 || a[i] != a[i-1] {
            b.push(a[i]);
        }
    }
    return b;
}

fn satisfies(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    let mut b_index = 0;
    for i in a {
        if *i == b[b_index] {
            b_index += 1;
            if b_index == b.len() {
                return true;
            }
        }
    }
    return b_index == b.len();
}

#[cfg(test)]
mod tests {
    use super::interweave;

    #[test]
    fn interweave_test() {
        let results = interweave(&vec![1,2,3], &vec![4,5,6]);
        assert!(results.contains(&vec![1,2,3,4,5,6]));
        assert!(results.contains(&vec![1,2,4,3,5,6]));
        assert!(results.contains(&vec![1,2,4,5,3,6]));
        assert!(results.contains(&vec![1,2,4,5,6,3]));
        assert!(results.contains(&vec![1,4,2,3,5,6]));
        assert!(results.contains(&vec![1,4,2,3,5,6]));
        assert!(results.contains(&vec![1,4,2,5,3,6]));
        assert!(results.contains(&vec![1,4,2,5,6,3]));
        assert!(!results.contains(&vec![2,1,3,4,5,6]));
    }

    #[test]
    fn test() {
        super::solve();
    }
}