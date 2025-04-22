use num::integer::Roots;

pub fn solve() {
    let mut count = 0;
    for i in 1..10000 {
        if i.sqrt() * i.sqrt() == i {
            continue;
        }
        let (_,b) = express_root(i);
        if b.len() % 2 == 1 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn express_root(num: u64) -> (u64, Vec<u64>) {
    let int_part = num.sqrt();
    let mut numerator = 1;
    let mut denominator_modifier = int_part;
    let mut repeating_part = vec![];
    let non_repeating_part = int_part;
    let mut points_encountered = vec![];
    loop {
        let numerator_modifier = denominator_modifier;
        let denominator = (num - numerator_modifier * numerator_modifier) / numerator;
        let new_int_part = (((num as f64).sqrt() + numerator_modifier as f64) / denominator as f64).floor() as u64;
        denominator_modifier = denominator * new_int_part - numerator_modifier;
        numerator = denominator;
        let new_point = (new_int_part, denominator_modifier, numerator);
        if points_encountered.contains(&new_point) {
            break;
        }
        points_encountered.push(new_point);
        repeating_part.push(new_int_part);
    }
    return (non_repeating_part, repeating_part);
}

#[cfg(test)]
mod tests {
    use super::{express_root, solve};

    #[test]
    fn test() {
        let (a,b) = express_root(2);
        assert_eq!(a,1);
        assert_eq!(b, vec![2]);
        let (a,b) = express_root(3);
        assert_eq!(a,1);
        assert_eq!(b, vec![1,2]);
        let (a,b) = express_root(5);
        assert_eq!(a,2);
        assert_eq!(b, vec![4]);
    }

    #[test]
    fn test_2() {
        solve();
    }
}