use number_theory::number_theory;

pub fn problem27() {
    let primes = number_theory::prime_sieve(1000);
    let mut max_prime_count = 0;
    let mut max_a:i64 = 0;
    let mut max_b:i64 = 0;
    for a in 0_i64..1000 {
        for b in 0_i64..1001 {
            let mut n = 0;
            let mut prime_count = 0;
            loop {
                match primes.binary_search(&((n*n + a*n + b) as u64)) {
                    Ok(_) => prime_count += 1,
                    Err(_) => break
                }
                n += 1;
            }
            if prime_count > max_prime_count {
                max_prime_count = prime_count;
                max_a = a;
                max_b = b;
            }
            n = 0;
            prime_count = 0;
            loop {
                match primes.binary_search(&((n*n + b - a*n) as u64)) {
                    Ok(_) => prime_count += 1,
                    Err(_) => break
                }
                n += 1;
            }
            if prime_count > max_prime_count {
                max_prime_count = prime_count;
                max_a = -a;
                max_b = b;
            }
            n = 0;
            prime_count = 0;
            loop {
                match primes.binary_search(&((n*n + a*n - b) as u64)) {
                    Ok(_) => prime_count += 1,
                    Err(_) => break
                }
                n += 1;
            }
            if prime_count > max_prime_count {
                max_prime_count = prime_count;
                max_a = a;
                max_b = -b;
            }
            n = 0;
            prime_count = 0;
            loop {
                match primes.binary_search(&((n*n - a*n - b) as u64)) {
                    Ok(_) => prime_count += 1,
                    Err(_) => break
                }
                n += 1;
            }
            if prime_count > max_prime_count {
                max_prime_count = prime_count;
                max_a = -a;
                max_b = -b;
            }
        }
    }
    println!("{} and {} generate {} primes in a row, and have a produce of {}", max_a, max_b, max_prime_count, max_a * max_b);
}