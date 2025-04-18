use num::{BigInt, Zero};
use number_theory::fraction::Fraction;

pub fn solve() {
    let mut frac = Fraction::<BigInt>::new(BigInt::new(num::bigint::Sign::Plus, vec![3]),BigInt::new(num::bigint::Sign::Plus, vec![2])).unwrap();
    let mut count = 0;
    for i in 2..1001 {
        frac = get_next_step(frac);
        if count_digits(frac.numerator.clone()) > count_digits(frac.denominator.clone()) {
            println!("{}",i);
            count += 1;
        }
        frac = Fraction::new(frac.numerator, frac.denominator).unwrap();
    }
    println!("{}", count);
}

fn get_next_step(frac: Fraction<BigInt>) -> Fraction<BigInt>  {
    return (frac + Fraction::one()).inverse()+Fraction::one();
}

fn count_digits(mut n: BigInt) -> BigInt {
    let mut count = BigInt::zero();
    let ten = BigInt::from(10);
    while n > BigInt::zero() {
        count = count + BigInt::from(1);
        n = n / &ten;
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test() {
        solve()
    }
}