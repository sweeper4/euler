use std::collections::HashSet;

pub fn divisors_include_one_and_n(n:u32) -> HashSet<u32> {
    let mut i = 1;
    let mut set = HashSet::new();
    while i <= n {
        if n % i == 0 {
            set.insert(i);
        }
        i += 1;
    }
    return set;
}

pub fn divisors_include_one(n:u32) -> HashSet<u32> {
    let mut i = 1;
    let mut set = HashSet::new();
    while i <= n/2 {
        if n % i == 0 {
            set.insert(i);
        }
        i += 1;
    }
    return set;
}