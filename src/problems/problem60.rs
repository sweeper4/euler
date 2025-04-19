use primes::{PrimeSet, Sieve};

pub fn solve() {
    let mut sieve = Sieve::new();
    let primes = sieve.iter();
    let mut cliques = vec![vec![]];
    for prime in primes {
        let mut new_cliques = vec![];
        for clique in cliques.iter_mut() {
            if fits_clique(prime, clique) {
                let mut new_clique = clique.clone();
                new_clique.push(prime);
                if new_clique.len() == 5 {
                    println!("{:?},{}", new_clique, new_clique.iter().map(|a| *a).sum::<u64>());
                    return;
                }
                new_cliques.push(new_clique);
            }
        }
        cliques.append(&mut new_cliques);
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