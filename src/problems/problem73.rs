use num::integer::gcd;

pub fn solve() {
    let mut count = 0;
    for i in 1..12001 {
        for j in i/3..i/2+1 {
            let gcd = gcd(i,j);
            if gcd == 1 && j * 3 > i && j * 2 < i {
                count += 1;
            }   
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}