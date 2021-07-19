use number_theory::number_theory;
use sorted_vec::SortedVec;

pub fn problem47() {
    let primes = number_theory::prime_sieve(150000);
    let mut i = 15;
    loop {
        let mut skip = 0;
        if !prime_factors_is_four(i+0, &primes) {
            skip = 1;
        };
        if !prime_factors_is_four(i+1, &primes) {
            skip = 2;
        };
        if !prime_factors_is_four(i+2, &primes) {
            skip = 3;
        };
        if !prime_factors_is_four(i+3, &primes) {
            skip = 4;
        };
        if skip == 0 {
            println!("It's {}", i);
            return;
        }
        i += skip;
    }
}

fn prime_factors_is_four(mut n: u64, primes: &SortedVec<u64>) -> bool {
    let mut count = 0;
    for prime in primes.iter() {
        if *prime > n {
            break;
        }
        if n % *prime == 0 {
            count += 1;
        }
        while n % *prime == 0 {
            n /= *prime;
        }
    }
    return count == 4;
}