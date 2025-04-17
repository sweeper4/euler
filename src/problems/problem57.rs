use big_int::BigInt;
use num_traits::PrimInt;
use number_theory::fraction::Fraction;

pub fn solve() {
    let mut frac = Fraction::<BigInt>::new(BigInt::from_string("3".to_owned()),BigInt::from_string("2".to_owned())).unwrap();
    let mut count = 0;
    for i in 2..1001 {
        frac = get_next_step(frac);
        if count_digits(frac.numerator) > count_digits(frac.denominator) {
            println!("{}",i);
            count += 1;
        }
    }
    println!("{}", count);
}

fn get_next_step<N: PrimInt>(frac: Fraction<N>) -> Fraction<N> {
    return (frac + Fraction::one()).inverse()+Fraction::one();
}

fn count_digits<N: PrimInt>(mut n: N) -> N {
    let mut count = N::zero();
    let ten = N::from(10).unwrap();
    while n > N::zero() {
        count = count + N::one();
        n = n / ten;
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