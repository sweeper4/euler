use num::{BigInt, bigint::Sign};
use number_theory::fraction::Fraction;

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

fn calculate_partial_sum(int_part: BigInt, repeating_part: Vec<u32>, term_count: usize) -> Fraction<BigInt> {
    let mut fraction = Fraction::new(BigInt::new(Sign::Plus, vec![0]),BigInt::new(Sign::Plus, vec![1])).unwrap();
    let mut i = term_count - 1;
    loop {
        if i == 0 {
            break;
        }
        fraction = fraction + Fraction::new(BigInt::new(Sign::Plus, vec![repeating_part[i % repeating_part.len()]]), BigInt::new(Sign::Plus, vec![1])).unwrap();
        fraction = fraction.inverse();
        i -= 1;
    }
    fraction = fraction + Fraction::new(int_part, BigInt::new(Sign::Plus, vec![1])).unwrap();
    return fraction;
}

#[cfg(test)]
mod tests {
    use num::{BigInt, bigint::Sign};
    use number_theory::fraction::Fraction;

    use crate::problems::problem65::calculate_partial_sum;

    use super::solve;

    #[test]
    fn calculate_partial_sum_works() {
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 1);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![1]), BigInt::new(Sign::Plus, vec![1])).unwrap());
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 2);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![3]), BigInt::new(Sign::Plus, vec![2])).unwrap());
        let a = calculate_partial_sum(BigInt::new(Sign::Plus, vec![1]), vec![2], 3);
        assert_eq!(a, Fraction::new(BigInt::new(Sign::Plus, vec![7]), BigInt::new(Sign::Plus, vec![5])).unwrap());
    }

    #[test]
    fn test() {
        solve();
    }
}