#![allow(dead_code)]
#![allow(unused_variables)]

use std::convert::TryInto;
use std::cmp::Ordering;
use std::ops::{Rem, Mul, Add, AddAssign};

#[derive(Debug, Clone)]
pub struct BigInt {
    pub signed: bool,
    pub num_vec: Vec<u32>
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal)
        }
        if self.signed && !other.signed {
            return Some(Ordering::Less);
        } else if !self.signed && other.signed {
            return Some(Ordering::Greater);
        } else if !self.signed {
            return self.num_vec.partial_cmp(&other.num_vec);
        } else {
            match self.num_vec.partial_cmp(&other.num_vec) {
                Some(Ordering::Greater) => Some(Ordering::Less),
                Some(Ordering::Less) => Some(Ordering::Greater),
                a => a
            }
        }
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        (self.signed == other.signed && self.num_vec == other.num_vec) || (self.is_zero() && other.is_zero())
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: BigInt) -> Self::Output {
        return self.add(other);
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        let result = self.clone().add(rhs);
        *self = BigInt {
            signed: result.signed,
            num_vec: result.num_vec
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: BigInt) -> Self::Output {
        return self.mul(other);
    }
}

impl Rem for BigInt {
    type Output = Self;

    fn rem(self, modulus: Self) -> Self::Output {
        // let mut temp = self;
        // while temp < modulus {
        //     temp 
        // }
        return self;
    }
}

impl BigInt {
    pub fn from_string(n: String) -> BigInt {
        if n.len() == 0 {
            return BigInt{
                signed: false,
                num_vec: vec![0]
            }
        }
        let mut vec: Vec<u32> = vec![];
        let signed = n.chars().nth(0).unwrap() == '-';

        let mut n_iter = n.chars().rev();
        let mut n_char = n_iter.next();

        let mut mul = 1;
        let mut sum = 0;

        while n_char.is_some() {
            if n_char.unwrap().is_digit(10) {
                sum += mul * n_char.unwrap().to_digit(10).unwrap();
                if mul == 100 {
                    mul = 1;
                    vec.insert(0, sum);
                    sum = 0;
                } else {
                    mul *= 10;
                }
            }

            n_char = n_iter.next();
        }

        if mul != 1 && sum > 0 {
            vec.insert(0, sum);
        }

        BigInt{
            signed: signed,
            num_vec: vec
        }
    }

    pub fn new(mut n: i32) -> BigInt {
        let mut vec: Vec<u32> = vec![];
        let signed = n < 0;
        if n < 0 {
            n *= -1;
        }
        match n {
            0 => BigInt{signed: signed, num_vec: vec![0]},
            mut i => {
                while i > 0 {
                    vec.insert(0,(i % 1000).try_into().unwrap());
                    i /= 1000;
                }
                BigInt{
                    signed: signed,
                    num_vec: vec
                }
            }
        }
    }

    pub fn show(&self) -> String {
        let mut show = "".to_string();
        if self.signed {
            show += "-";
        }
        for (i, val) in self.num_vec.iter().enumerate() {
            let mut string = val.to_string();
            if i > 0 {
                while string.len() < 3 {
                    string = "0".to_owned() + &string;
                }
            }
            show += &string;
        }
        return show;
    }
    
    pub fn show_formatted(&self) -> String {
        let mut show = "".to_string();
        show += "BigInt { ";
        if self.signed {
            show += "-";
        }
        for (i, val) in self.num_vec.iter().enumerate() {
            let mut string = val.to_string();
            if i > 0 {
                show += ",";
                while string.len() < 3 {
                    string = "0".to_owned() + &string;
                }
            }
            show += &string;
        }
        show += " }";
        return show;
    }

    pub fn add(self, other: BigInt) -> BigInt {
        if self.signed != other.signed {
            if self.signed {
                return BigInt{
                    signed: other.signed,
                    num_vec: other.num_vec
                }.sub(BigInt{
                    signed: !self.signed,
                    num_vec: self.num_vec
                });
            } else {
                return BigInt{
                    signed: self.signed,
                    num_vec: self.num_vec
                }.sub(BigInt{
                    signed: !other.signed,
                    num_vec: other.num_vec
                });
            }
        }
        
        return BigInt{
            signed: self.signed,
            num_vec: BigInt::remove_leading_zeros(BigInt::add_array(self.num_vec, other.num_vec))
        }
    }

    pub fn sub(self, other: BigInt) -> BigInt {
        if self.signed != other.signed {
            return BigInt{
                signed: self.signed,
                num_vec: BigInt{
                    signed: false,
                    num_vec: self.num_vec
                }.add(
                    BigInt{
                        signed: false,
                        num_vec: other.num_vec
                    }
                ).num_vec
            };
        }
        if self.num_vec == other.num_vec {
            return BigInt{
                signed: false,
                num_vec: vec![0]
            }
        }

        let signed: bool;
        if self.num_vec < other.num_vec {
            signed = !self.signed;
        } else {
            signed = self.signed;
        }

        return BigInt{
            signed: signed,
            num_vec: BigInt::remove_leading_zeros(BigInt::sub_array(self.num_vec, other.num_vec))
        }
    }

    pub fn mul(self, other: BigInt) -> BigInt {
        let signed = self.signed != other.signed;
        
        return BigInt{
            signed: signed,
            num_vec: BigInt::remove_leading_zeros(BigInt::mul_array(self.num_vec, other.num_vec))
        };
    }

    pub fn div(self, other: BigInt) -> BigInt {
        return self;
    }

    /**
        Does not support negative exponents currently
    */
    pub fn pow(self, mut pow: u32) -> BigInt {
        let mut product = BigInt::new(1);
        let mut temp = self;
        loop {
            if pow % 2 != 0 {
                product = product * temp.clone();
            }
            temp = temp.clone() * temp.clone();
            if pow == 0 {
                return product;
            }
            pow /= 2;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.num_vec.len() == 1 && self.num_vec[0] == 0
    }

    fn remove_leading_zeros(num_vec: Vec<u32>) -> Vec<u32> {
        let mut new_vec = num_vec;
        while new_vec.len() > 1 && new_vec[0] == 0 {
            new_vec.remove(0);
        }
        return new_vec;
    }

    fn add_array(num1:Vec<u32>, num2:Vec<u32>) -> Vec<u32> {
        let mut carry = 0;
        let mut new_vec = vec![];
        let mut a = num1.iter().rev();
        let mut b = num2.iter().rev();
        let mut a_val = a.next();
        let mut b_val = b.next();
        while a_val.is_some() && b_val.is_some() {
            let sum = a_val.unwrap() + b_val.unwrap() + carry;
            new_vec.insert(0, sum % 1000);
            carry = sum / 1000;
            a_val = a.next();
            b_val = b.next();
        }
        while a_val.is_some() {
            let sum = a_val.unwrap() + carry;
            new_vec.insert(0, sum % 1000);
            carry = sum / 1000;
            a_val = a.next();
        }
        while b_val.is_some() {
            let sum = b_val.unwrap() + carry;
            new_vec.insert(0, sum % 1000);
            carry = sum / 1000;
            b_val = b.next();
        }
        while carry > 0 {
            new_vec.insert(0, carry % 1000);
            carry /= 1000;
        }
        return new_vec;
    }

    fn sub_array(num1:Vec<u32>, num2:Vec<u32>) -> Vec<u32> {
        let mut a;
        let mut b;
        let mut carry = 0;
        let mut new_vec = vec![];
        if num1 < num2 {
            a = num2.iter().rev();
            b = num1.iter().rev();
        } else {
            a = num1.iter().rev();
            b = num2.iter().rev();
        }
        let mut a_val = a.next();
        let mut b_val = b.next();
        while a_val.is_some() || b_val.is_some() {
            let sum;
            if a_val.unwrap() < b_val.unwrap() {
                sum = 1000 + a_val.unwrap() - b_val.unwrap() - carry;
                carry = 1;
            } else {
                sum = a_val.unwrap() - b_val.unwrap() - carry;
                carry = 0;
            }
            new_vec.insert(0, sum % 1000);
            a_val = a.next();
            b_val = b.next();
        }
        return new_vec;
    }

    fn mul_array(num1:Vec<u32>, num2:Vec<u32>) -> Vec<u32> {
        let mut new_vec = vec![];
        let mut a = num1.iter().rev();
        let mut a_val = a.next();
        let mut a_index = 0;
        while a_val.is_some() {
            let mut b = num2.iter().rev();
            let mut b_val = b.next();
            let mut b_index = 0;
            let mut carry = 0;
            while b_val.is_some() {
                let mut product = a_val.unwrap() * b_val.unwrap() + carry;
                if new_vec.len() == a_index + b_index {
                    new_vec.insert(0, product % 1000);
                    carry = product / 1000;
                } else {
                    let update_index = new_vec.len() - 1 - a_index - b_index;
                    product += new_vec[update_index];
                    new_vec[update_index] = product % 1000;
                    carry = product / 1000;
                }
                b_val = b.next();
                b_index += 1;
            }
            while carry > 0 {
                new_vec.insert(0, carry % 1000);
                carry /= 1000;
            }
            a_val = a.next();
            a_index += 1;
        }
        return new_vec;
    }
}

#[cfg(test)]
mod test {
    use super::BigInt;

    #[test]
    fn big_int_pow_works() {
        assert_eq!(BigInt::new(5).pow(5), BigInt::from_string("3125".to_owned()));
        assert_eq!(BigInt::new(99).pow(99), BigInt::from_string("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899".to_owned()));
    }
}