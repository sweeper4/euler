use number_theory::number_theory::num_length;

pub fn solve() {
    let mut power: u32 = 0;
    let mut count = 0;
    loop {
        power += 1;
        let mut base: u128 = 0;
        loop {
            base += 1;
            let result = base.pow(power);
            let len = num_length(result, 10) as u32;
            if len < power {
                continue;
            } else if len == power {
                count += 1;
            } else {
                break;
            }
        }
        if power > 25 {
            break;
        }
    }
    println!("{}",count);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test] 
    fn test() {
        solve();
    }
}