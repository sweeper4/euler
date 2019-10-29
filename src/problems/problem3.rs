pub fn get_primes_less_than(max: usize) -> Vec<usize> {
    let mut ints : Vec<usize> = (2..max).collect();
    let mut primes : Vec<usize> = Vec::new();
    let upper_bound = get_sqrt_aprox(max);
    let mut cont = true;
    while cont {
        let prime = ints[0];
        ints = ints.drain(..).filter(|x| {
            *x % prime != 0
        }).collect();
        primes.push(prime);
        if prime >= upper_bound || ints.len() == 0  {
            cont = false;
        }

    }
    primes.extend(ints.iter().cloned());
    return primes
}

fn get_sqrt_aprox(base: usize) -> usize {
    let mut guess = base / 2;
    let mut old_guess = base;
    while guess.overflowing_mul(guess).0 > base {
        old_guess = guess;
        guess = (base/guess + guess) / 2;
    }
    return old_guess
}

pub fn get_max_factor(number: usize) -> usize {
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