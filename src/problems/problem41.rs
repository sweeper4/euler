use primes;
use std::convert::TryInto;

pub fn problem41() {
    println!("Just another pandigital prime problem, answers {}", solve_the_thing());
}

fn solve_the_thing() -> u64 {
    let mut prime_set = primes::PrimeSet::new();
    let mut primes = vec![];
    for prime in prime_set.iter() {
        if prime > 987654321 {
            break;
        }
        primes.push(prime);
    }
    for prime in primes.iter().rev() {
        if number_theory::number_theory::is_pandigital(*prime, prime.to_string().len().try_into().unwrap()) {
            return *prime;
        }
    }
    return 2;
}