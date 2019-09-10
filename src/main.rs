#![feature(drain_filter)]
fn main() {
    println!("sum of the threes and fives is {} lower than {}",multiples_of_3_5(1000),1000);
    sum_evens_less_than(4000000);
    println!("The max prime factor of  {} is {}",600851475143_u64,get_max_factor(600851475143));
    println!(" max palindrome {}",max_palindrome_multiple_between(100,999));
    println!(" the smallest multiple of numbers less than {} is {}",10,smallest_multiple(10));
    println!(" the smallest multiple of numbers less than {} is {}",20,smallest_multiple(20));
}

fn multiples_of_3_5(max : u32) -> u32 {
    let correct = max - 1;
    let sum_threes = 3 * sum_to(correct/3);
    let sum_fives = 5 * sum_to(correct/5);
    let sum_fifteens = 15 * sum_to(correct/15);
    sum_threes + sum_fives - sum_fifteens
}

fn sum_to(max: u32) -> u32 {
    (max*(max+1))/2
}

fn sum_evens_less_than( max: usize){
    let mut fb_generator = Fb::new();
    let mut i = 0;
    let mut total = 0;
    while fb_generator.get(i) < max {
        if fb_generator.get(i) % 2 == 0 {
            total = total + fb_generator.get(i);
        }
        i = i + 1;
    }
    println!("total of even fibonacci numbers  = {}",total);
}

struct Fb {
    cache: Vec<usize>
}

impl Fb {
    fn get( &mut self,index: usize) -> usize {
        match index {
            0 => 1_usize,
            1 => 2_usize,
            _ =>
                match self.cache.get(index - 2) {
                    Some( a ) => {
                        *a },
                    None => {
                                let r = self.get(index - 2) + self.get( index - 1 );
                                self.cache.push(r);
                                r }
                }
        }
    }

    fn new() -> Fb {
        Fb{
            cache: Vec::new()}
    }
}


fn get_primes_less_than(max: usize) -> Vec<usize> {
    let mut ints : Vec<usize> = (2..max).collect();
    let mut primes : Vec<usize> = Vec::new();
    let upper_bound = get_sqrt_aprox(max);
    let mut cont = true;
    while cont {
        let prime = ints[0];
        ints.drain_filter(|x| *x % prime == 0);
        primes.push(prime);
        if prime >= upper_bound || ints.len() == 0  {
            cont = false;
        }

    }
    primes.extend(ints.iter().cloned());
    primes
}


fn get_sqrt_aprox(base: usize) -> usize{
    let mut guess = base / 2;
    let mut old_guess = base;
    while guess.overflowing_mul(guess).0 > base {
        old_guess = guess;
        guess = (base/guess + guess) / 2;
    }
    old_guess
}

fn get_max_factor(number: usize) -> usize{
    match get_prime_factors(number).last(){
        Some(prime) => *prime,
        None => 0
    }
                                        
}

fn get_prime_factors(number: usize) -> Vec<usize> {
    let sqrt = get_sqrt_aprox(number);
    let mut factors = Vec::new();
    let primes = get_primes_less_than(sqrt);
    for prime in primes {
        if number % prime == 0 {
            factors.push(prime);
        }
    }
    factors
}

fn max_palindrome_multiple_between(min:usize,max:usize) -> usize{
    let mut palindromes = Vec::new();
    for x in min..max+1 {
        for y in min..max+1 {
            if is_Palindrome(x*y) {
                palindromes.push(x*y);
            }
        }
    }
    palindromes.iter().cloned().fold(0_usize,|max,a| if a > max { a}else{ max })

}

fn is_Palindrome( number: usize) -> bool {
    number == reverse(number)    
}

fn reverse(number:usize) -> usize{
    let digits = DigitIter::new(number);
    let mut value = 0;
    let mut counter = 0;
    for digit in digits{
        value = (value * 10) + digit;
        counter = counter + 1;
    }
    value
}


struct DigitIter{
    number: usize,
    base: usize,
}

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            None
        } else {
            let r = self.number % self.base;
            self.number /= self.base;
            Some(r)
        }
    }

}
impl DigitIter {
    fn new(num: usize) -> DigitIter {
        DigitIter{number: num, base:10}
    }
}

fn smallest_multiple(max:usize) -> usize {
    get_primes_less_than(max).iter().product()
}
