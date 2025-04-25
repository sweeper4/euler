use std::{cmp::max, fs::File, io::{BufRead, BufReader}};

use regex::Regex;

pub fn solve() {
    let file = File::open("src/assets/problem67triangle.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut memoized = vec![];
    for _ in 0..2 {
        memoized.push(0);
    }
    for line in lines {
        let line = line.unwrap();
        let regex = Regex::new(r"\d+").unwrap();
        let mut new_memo = vec![0];
        for (i, digit) in regex.find_iter(&line).enumerate() {
            let digit = digit.as_str().parse::<u32>().unwrap();
            new_memo.push(digit + max(memoized[i], memoized[i+1]));
        }
        new_memo.push(0);
        memoized = new_memo;
    }
    let max = memoized.iter().max().unwrap();
    println!("{}", max);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}