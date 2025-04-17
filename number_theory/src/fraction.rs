use std::ops::{Add, Mul, Div, Rem};

use num_traits::{Zero, One};

use crate::number_theory::gcd;

pub struct Fraction<N: Mul<Output = N> + Div<Output = N> + Zero + One + PartialOrd + Rem<Output = N> + Clone> {
    pub numerator: N,
    pub denominator: N
}

impl<N: Mul<Output = N> + Div<Output = N> + Zero + One + PartialOrd + Rem<Output = N> + Clone> Fraction<N> {
    pub fn new(numerator: N, denominator: N) -> Option<Fraction<N>> {
        if denominator.is_zero() {
            return None;
        }
        return Some(Fraction {numerator, denominator});
    }

    pub fn inverse(self) -> Fraction<N> {
        return Fraction { numerator: self.denominator, denominator: self.numerator};
    }

    pub fn one() -> Fraction<N> {
        return Fraction { numerator: N::one(), denominator: N::one() }
    }
}

impl<N: Mul<Output = N> + Div<Output = N> + Zero + One + PartialOrd + Rem<Output = N> + Clone> Add for Fraction<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator_1 = self.numerator * rhs.denominator;
        let numerator_2 = rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        let common = gcd(&gcd(&numerator_1, &numerator_2), &denominator);
        let numerator_1 = numerator_1 / common;
        let numerator_2 = numerator_2 / common;
        let denominator = denominator / common;
        let numerator = numerator_1 + numerator_2;
        let common = gcd(&numerator, &denominator);
        let numerator = numerator / common;
        let denominator = denominator / common;
        return Fraction {numerator, denominator};
    }
}

impl<N: Mul<Output = N> + Div<Output = N> + Zero + One + PartialOrd + Rem<Output = N> + Clone> Mul for Fraction<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        let common = gcd(&numerator, &denominator);
        let numerator = numerator / common;
        let denominator = denominator / common;
        return Fraction {numerator, denominator};
    }
}

#[cfg(test)]
mod test {
    use super::Fraction;

    #[test]
    fn fraction_creation() {
        let frac = Fraction::<u64>::new(1, 2);
        assert!(frac.is_some());
        let frac = frac.unwrap();
        assert_eq!(frac.numerator, 1);
        assert_eq!(frac.denominator, 2);
    }

    #[test]
    fn fraction_addition() {
        let a = Fraction::<u64>::new(1, 2).unwrap();
        let b = Fraction::<u64>::new(3, 4).unwrap();
        let c: Fraction<u64> = a + b;
        assert_eq!(c.numerator, 5);
        assert_eq!(c.denominator, 4);
    }

    #[test]
    fn fraction_multiplication() {
        let a = Fraction::<u64>::new(1, 2).unwrap();
        let b = Fraction::<u64>::new(3, 4).unwrap();
        let c: Fraction<u64> = a * b;
        assert_eq!(c.numerator, 3);
        assert_eq!(c.denominator, 8);
    }

    #[test]
    fn fraction_inverse() {
        let a = Fraction::<u64>::new(1, 2).unwrap();
        let a = a.inverse();
        assert_eq!(a.numerator, 2);
        assert_eq!(a.denominator, 1);
    }
}