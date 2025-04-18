pub fn solve() {
    let mut current = 1;
    let mut diagonal_count = 1;
    let mut prime_count = 0;
    let mut step_size = 0;
    loop {
        step_size += 2;
        for _ in 0..4 {
            current += step_size;
            diagonal_count += 1;
            if number_theory::number_theory::is_prime(current) {
                prime_count += 1;
            }
        }
        if prime_count * 10 < diagonal_count {
            println!("{}", step_size+1);
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}