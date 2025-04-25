use num::{integer::Roots, BigInt, FromPrimitive, Zero};
use number_theory::number_theory::{calculate_partial_sum, continued_fraction_of_sqrt};

pub fn solve() {
    let mut biggest_p = BigInt::zero();
    let mut biggest_i = 0;
    for i in 1..1001 {
        if i.sqrt() * i.sqrt() == i {
            continue;
        }
        let (a,b) = continued_fraction_of_sqrt(i);
        let mut j = 1;
        loop {
            let partial = calculate_partial_sum(BigInt::from_u64(a).unwrap(), b.clone(), j);
            let p = partial.numerator;
            let q = partial.denominator;
            if &p * &p - i * &q * &q == BigInt::from_u64(1).unwrap() {
                if p > biggest_p {
                    biggest_p = p;
                    biggest_i = i;
                }
                break;
            }
            j += 1;
        }
    }
    println!("{}", biggest_i);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}