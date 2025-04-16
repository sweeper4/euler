use num_traits::PrimInt;

pub fn solve() {
    let mut count = 0;
    for i in 0..10_001_u128 {
        let mut num = i;
        let mut is_lychrel = true;
        for _ in 0..50 {
            num = lychrel_step(num);
            if is_palindrome(num) {
                is_lychrel = false;
                break;
            }
        }
        if is_lychrel {
            count += 1;
        }
    }
    println!("{}", count);
}

fn lychrel_step<N: PrimInt>(mut num: N) -> N {
    let ten: N = N::from(10).unwrap();
    let zero = N::zero();
    let forward_num = num;
    let mut backward_num = zero;
    while num > zero {
        backward_num = backward_num * ten + num % ten;
        num = num / ten;
    }
    return forward_num + backward_num;
}

fn is_palindrome<N: PrimInt>(mut num: N) -> bool {
    let ten: N = N::from(10).unwrap();
    let zero = N::zero();
    let mut digits = vec![];
    while num > zero {
        digits.push(num % ten);
        num = num / ten;
    }
    for i in 0..digits.len()/2 {
        if digits[i] != digits[digits.len()-1-i] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}