fn max_palindrome_multiple_between(min:usize, max:usize) -> usize {
    let mut palindromes = Vec::new();
    for x in min..max+1 {
        for y in min..max+1 {
            if is_palindrome(x*y) {
                palindromes.push(x*y);
            }
        }
    }
    return palindromes.iter().cloned().fold(0_usize,|max,a| if a > max { a } else{ max })
}

fn is_palindrome( number: usize) -> bool {
    return number == reverse(number);
}

fn reverse(number:usize) -> usize{
    let digits = DigitIter::new(number);
    let mut value = 0;
    let mut counter = 0;
    for digit in digits {
        value = (value * 10) + digit;
        counter = counter + 1;
    }
    return value;
}

struct DigitIter{
    number: usize,
    base: usize,
}

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            return None
        } else {
            let r = self.number % self.base;
            self.number /= self.base;
            return Some(r)
        }
    }

}
impl DigitIter {
    fn new(num: usize) -> DigitIter {
        return DigitIter{number: num, base:10}
    }
}

pub fn solve() {
    println!("Max palindrome {}",max_palindrome_multiple_between(100,999));
}