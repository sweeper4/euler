use number_theory::number_theory;

pub fn solve() {
    let primes = number_theory::prime_sieve(10000);
    let mut i = 7;
    loop {
        i += 2;
        if primes.contains(&i) {
            continue;
        }
        let mut test = true;
        for prime in primes.iter() {
            if *prime == 2 {
                continue;
            }
            if *prime > i {
                break;
            }
            let root = (((i - *prime) / 2) as f64).sqrt() as u64;
            if root * root * 2 + *prime == i {
                test = false;
            }
        }
        if test {
            println!("There is no prime p and square s such that {} = p + 2 * s", i);
            return;
        }
    }
}