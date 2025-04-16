use std::collections::HashSet;
use std::collections::BTreeMap;

pub fn solve() {
    println!("The number of distinct values of all a^b where 2 <= a <= 100 and 2 <= b <= 100 is {}", count_distinct_powers(100,100));
}

fn count_distinct_powers(base:u64, power:u64) -> u64 {
    let mut seen:HashSet<BTreeMap<u64,u64>> = HashSet::new();
    let mut count = 0;
    let mut i = 2;
    while i <= base {
        let mut j = 2;
        while j <= power {
            let prime_factors = get_prime_factors(i,j);
            if !seen.contains(&prime_factors) {
                count += 1;
                seen.insert(prime_factors);
            }
            j += 1;
        }
        i += 1;
    }
    return count;
}

fn get_prime_factors(i:u64, j:u64) -> BTreeMap<u64,u64> {
    let factors = number_theory::number_theory::prime_factors_of(i.into());
    let mut final_factors = BTreeMap::new();
    for (a, b) in factors {
        final_factors.insert(a, b * j);
    }
    return final_factors;
}