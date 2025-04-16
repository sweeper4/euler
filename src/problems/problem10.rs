pub fn solve() {
    println!("The sum of primes less than {} is {}", 2000000, sum_sequence(get_primes_less_than(2000000)));
}

fn get_primes_less_than(max: u64) -> Vec<u64> {
    let mut ints : Vec<u64> = (2..max).collect();
    let mut primes : Vec<u64> = Vec::new();
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

fn get_sqrt_aprox(base: u64) -> u64 {
    let mut guess = base / 2;
    let mut old_guess = base;
    while guess.overflowing_mul(guess).0 > base {
        old_guess = guess;
        guess = (base/guess + guess) / 2;
    }
    return old_guess
}

fn sum_sequence(seq: Vec<u64>) -> u64 {
    let mut sum = 0;
    for value in seq {
        sum += value;
    }
    return sum;
}