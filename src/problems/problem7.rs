pub fn solve() {
    println!("The 10,001th prime is {}", get_nth_prime(10001));
}

fn get_nth_prime(n: usize) -> i32 {
    if n == 1 {
        return 2;
    }
    let mut i = 1;
    let mut primes : Vec<i32> = vec![2];
    primes.push(2);
    while primes.len() <= n {
        i = i + 2;
        let mut is_prime: bool = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    return primes[n];
}