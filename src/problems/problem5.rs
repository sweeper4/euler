fn smallest_multiple(max:usize) -> usize {
    let min_prime : usize = get_primes_less_than(max).iter().product();
    let mut i = 1;
    while (2_usize..max).map(|x| (i * min_prime) % x == 0 )
        .filter(|x| *x ).collect::<Vec<bool>>().len() != max-2 {
        i = i + 1;
    }
    return i * min_prime;
}

fn get_primes_less_than(max: usize) -> Vec<usize> {
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

pub fn problem5() {
    println!("The smallest multiple of numbers less than {} is {}",20,smallest_multiple(20));
}
