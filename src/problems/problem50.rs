use number_theory::number_theory;

pub fn problem50() {
    println!("The longest series of consequetive primes that sums to a prime under 1,000,000 sums to {}", find_largest_prime_sum_of_sequential_primes_under(1000000));
}

fn find_largest_prime_sum_of_sequential_primes_under(upper_bound: u64) -> u64 {
    let primes = number_theory::prime_sieve(upper_bound);

    let mut max_sequence_length = 0;
    let mut sum = 0;
    for prime in primes.iter() {
        sum += prime;
        max_sequence_length += 1;
        if sum > upper_bound {
            break;
        }
    }

    if primes.contains(&sum) {
        return sum;
    }

    while max_sequence_length > 0 {
        let mut offset = 0;
        loop {
            let trial_sum = primes.iter().skip(offset).take(max_sequence_length).fold(0, |acc, &x| acc + x);
            if trial_sum > 1000000 || offset + max_sequence_length > 600 {
                break;
            }
            if primes.contains(&trial_sum) {
                return trial_sum;
            }
            offset += 1;
        }
        max_sequence_length -= 1;
    }

    return max_sequence_length as u64;
}