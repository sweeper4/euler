use std::ops::{ Mul, MulAssign };
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    pub sign: bool,
    pub numerator: u64,
    pub denominator: u64,
}

impl Fraction {
    pub fn show(self) -> String {
        if self.sign {
            return format!("-{:?}/{:?}", self.numerator, self.denominator);
        } else {
            return format!("{:?}/{:?}", self.numerator, self.denominator);
        }
    }

    pub fn new(sign:bool, num:u64, denum:u64) -> Fraction {
        let common_divisor = gcd(num,denum);
        return Fraction {
            sign: sign,
            numerator: num/common_divisor,
            denominator: denum/common_divisor,
        };
    }

    pub fn mul(self, other:Fraction) -> Fraction {
        /*
        let sign = self.sign ^ other.sign;
        let mut keys = vec![];
        for key in self.numerator.keys() {
            keys.push(*key);
        }
        for key in self.denominator.keys() {
            keys.push(*key);
        }
        for key in other.numerator.keys() {
            keys.push(*key);
        }
        for key in other.denominator.keys() {
            keys.push(*key);
        }
        let mut new_num: HashMap<u64,u64> = HashMap::new();
        let mut new_denum: HashMap<u64,u64> = HashMap::new();
        for key in keys.iter() {
            let numerator_count = *self.numerator.entry(*key).or_default() + *other.numerator.entry(*key).or_default();
            let denominator_count = *self.denominator.entry(*key).or_default() + *other.denominator.entry(*key).or_default();
            if numerator_count > denominator_count {
                new_num.insert(*key, numerator_count - denominator_count);
            } else {
                new_denum.insert(*key, denominator_count - numerator_count);
            }
        }
        return Fraction {
            sign: sign,
            numerator: new_num,
            denominator: new_denum,
        }
        */
        let common_divisor = gcd(self.numerator * other.numerator, self.denominator * other.denominator);
        return Fraction {
            sign: self.sign ^ other.sign,
            numerator: self.numerator * other.numerator / common_divisor,
            denominator: self.denominator * other.denominator / common_divisor,
        };
    }

    pub fn cmp(self: &Fraction, other: &Fraction) -> Ordering {
        if self.sign ^ other.sign {
            if self.sign {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        if self.numerator * other.denominator < other.numerator * self.denominator {
            return Ordering::Less;
        }
        if self.numerator * other.denominator > other.numerator * self.denominator {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs:Fraction) -> Self::Output {
        return self.mul(rhs);
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Fraction) {
        let result = self.clone().mul(rhs);
        *self = Fraction {
            sign: result.sign,
            numerator: result.numerator,
            denominator: result.denominator,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other:&Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false
        }
    }
}

impl Eq for Fraction { }

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cmp(other);
    }
}

pub fn gcd(a:u64, b:u64) -> u64 {
    if b <= 1 {
        return 1;
    }
    if a < b {
        return gcd(b,a);
    }
    if a % b == 0 {
        return b;
    }
    return gcd(b, a % b);
}
