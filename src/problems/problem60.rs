use primes::{PrimeSet, Sieve};

pub fn solve() {
    let mut sieve = Sieve::new();
    let primes = sieve.iter();
    let mut cliques = vec![];
    for prime in primes {
        for clique in cliques.iter_mut() {
            if fits_clique(prime, clique) {
                clique.push(prime);
                if clique.len() == 5 {
                    println!("{:?},{}", clique, clique.iter().map(|a| *a).sum::<u64>());
                    return;
                }
            }
        }
        cliques.push(vec![prime]);
    }

}

fn fits_clique(prime: u64, clique: &Vec<u64>) -> bool {
    for other_prime in clique {
        let num:u64 = format!("{}{}",prime,other_prime).parse().unwrap();
        let num2:u64 = format!("{}{}",other_prime,prime).parse().unwrap();
        if !primes::is_prime(num) || !primes::is_prime(num2) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}