use num::BigInt;

pub fn solve() {
    let mut biggest_total = 0;
    for i in (1..100).rev() {
        for j in (1..100).rev() {
            let base = BigInt::new(num::bigint::Sign::Plus, vec![i]);
            let power = base.pow(j);
            let mut total = 0;
            for char in power.to_str_radix(10).chars() {
                total += (char as u8 - '0' as u8) as u32;
            }
            if total > biggest_total {
                biggest_total = total;
            }
        }
    }
    println!("{}", biggest_total);
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}