pub fn smallest_multiple(max:usize) -> usize {
    let min_prime : usize = crate::problems::problem3::get_primes_less_than(max).iter().product();
    let mut i = 1;
    while (2_usize..max).map(|x| (i * min_prime) % x == 0 )
        .filter(|x| *x ).collect::<Vec<bool>>().len() != max-2 {
        i = i + 1;
    }
    return i * min_prime;
}