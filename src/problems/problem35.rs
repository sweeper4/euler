use std::collections::HashSet;

pub fn problem35() {
    println!("There are {} circular primes below 1,000,000", count_circular_primes_below(1000000));
}

fn count_circular_primes_below(n:u64) -> usize {
    let mut primes = primes::PrimeSet::new();
    let mut primes_under_n = HashSet::new();
    let mut primes_seen = HashSet::new();
    let mut count = 0;
    for prime in primes.iter() {
        if prime >= n {
            break;
        }
        primes_under_n.insert(prime);
    }
    for prime in &primes_under_n {
        if !primes_seen.contains(prime) {
            count += get_circular_primes(*prime, &primes_under_n, &primes_seen);
        }
        primes_seen.insert(*prime);
    }
    return count;
}

fn get_circular_primes(prime:u64, primes:&HashSet<u64>, primes_seen:&HashSet<u64>) -> usize {
    let mut temp_prime = prime;
    let mut offset = 1;
    while temp_prime > 0 {
        temp_prime /= 10;
        offset *= 10;
    }
    offset /= 10;
    temp_prime = prime;
    let mut count = 1;
    temp_prime = temp_prime / 10 + (temp_prime % 10) * offset;
    while temp_prime != prime {
        if !primes.contains(&temp_prime) || primes_seen.contains(&temp_prime) {
            return 0;
        }
        count += 1;
        temp_prime = temp_prime / 10 + (temp_prime % 10) * offset;
    }
    return count;
}