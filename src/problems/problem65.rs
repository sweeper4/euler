use num::{BigInt, bigint::Sign};
use number_theory::number_theory::calculate_partial_sum;

pub fn solve() {
    let mut repeating_part = vec![];
    for i in 0..100 {
        repeating_part.push(
            if i % 3 == 1 {
                (i + 2) / 3 * 2
            } else {
                1
            }
        );
    }
    let partial_sum = calculate_partial_sum(BigInt::new(Sign::Plus, vec![2]), repeating_part, 100);
    let a = partial_sum.numerator;
    let sum: u64 = a.to_str_radix(10).chars().map(|x| x.to_digit(10).unwrap() as u64).sum();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}