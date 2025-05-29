use num::integer::gcd;

pub fn solve() {
    let mut best_num: u64 = 0;
    let mut best_denum: u64 = 1;
    for d in 2..1_000_001 {
        let mut numerator = 3 * d / 7;
        if numerator > 0 {
            numerator -= 1;
        }
        let gcd = gcd(d, numerator);
        if best_num * d < numerator * best_denum && gcd == 1 {
            best_denum = d;
            best_num = numerator;
        }
    }
    println!("{}/{}", best_num, best_denum);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}