use num::integer::gcd;

pub fn solve() {
    let limit = 1_500_000_u64;
    let mut triple_lens = vec![];
    let mut n = 0;
    loop {
        n += 1;
        if 2 * n * n > limit {
            break;
        }
        let mut m = n;
        loop {
            m += 1;
            if 2 * n * m > limit {
                break;
            }
            if gcd(m,n) != 1 {
                continue;
            }
            if m % 2 == 1 && n % 2 == 1 {
                continue;
            }
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            if a + b + c <= limit {
                triple_lens.push(a + b + c);
            }
        }
    }

    triple_lens.sort();

    let mut valid_ls = 0;
    let mut seen_ls = vec![];
    for i in 0..triple_lens.len() {
        let mut l = 0;
        if seen_ls.contains(&triple_lens[i]) {
            continue;
        }
        loop {
            l += triple_lens[i];
            if l > limit {
                break;
            }
            if seen_ls.contains(&l) {
                continue;
            }
            seen_ls.push(l);
            let mut divisors = 0;
            for j in 0..triple_lens.len() {
                if triple_lens[j] > l {
                    break;
                }
                if l % triple_lens[j] == 0 {
                    divisors += 1;
                }
            }
            if divisors == 1 {
                valid_ls += 1;
            }
        }
        // println!("{}/{}", i, triple_lens.len());
    }

    println!("{}", valid_ls);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}