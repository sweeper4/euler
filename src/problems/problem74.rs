use std::{collections::HashMap};

pub fn solve() {
    let factorial: Vec<u64> = vec![1,1,2,6,24,120,720,5040,40320,362880];
    let mut chain = HashMap::new();
    let mut count = 0;
    for i in 1..1_000_001 {
        let mut seen = vec![];
        let (a,b) = get_lens(i, &mut chain, &factorial, &mut seen);
        if a.len() + b.len() == 60 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn get_lens(val: u64, chain: &mut HashMap<u64, (Vec<u64>, Vec<u64>)>, factorial: &Vec<u64>, seen: &mut Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    if chain.contains_key(&val) {
        let (non_repeating, repeating) = chain.get(&val).unwrap();
        return (non_repeating.clone(), repeating.clone());
    }
    seen.push(val);
    let new_val = advance(val, factorial);
    if seen.contains(&new_val) {
        let mut repeating = vec![];
        let non_repeating = vec![];
        let mut in_repeating = false;
        for a in seen {
            if *a == new_val {
                in_repeating = true;
            }
            if in_repeating {
                repeating.push(*a);
            }
        }
        repeating.rotate_right(1);
        chain.insert(val, (non_repeating.clone(), repeating.clone()));
        return (non_repeating, repeating);
    } else {
        let (mut non_repeating, mut repeating) = get_lens(new_val, chain, factorial, seen);
        if repeating.contains(&val) {
            let index = repeating.iter().position(|a| *a == val).unwrap();
            repeating.rotate_left(index);
            chain.insert(val, (non_repeating.clone(), repeating.clone()));
            return (non_repeating, repeating);
        } else {
            non_repeating.insert(0, val);
            chain.insert(val, (non_repeating.clone(), repeating.clone()));
            return (non_repeating, repeating);
        }
    }
}

fn advance(mut val: u64, factorial: &Vec<u64>) -> u64 {
    let mut total = 0;
    while val > 0 {
        total += factorial[val as usize % 10];
        val /= 10;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::problems::problem74::advance;

    use super::{solve, get_lens};

    #[test]
    fn test() {
        solve();
    }

    #[test]
    fn advance_test() {
        let factorial: Vec<u64> = vec![1,1,2,6,24,120,720,5040,40320,362880];
        assert_eq!(145, advance(145, &factorial));
    }

    #[test]
    fn get_lens_test() {
        let mut chain = HashMap::new();
        let factorial: Vec<u64> = vec![1,1,2,6,24,120,720,5040,40320,362880];
        let mut seen = vec![];
        let (non_repeating, repeating) = get_lens(69, &mut chain, &factorial, &mut seen);
        assert_eq!(2, non_repeating.len());
        assert_eq!(69, non_repeating[0]);
        assert_eq!(363600, non_repeating[1]);
        assert_eq!(3, repeating.len());
        assert_eq!(1454, repeating[0]);
        assert_eq!(169, repeating[1]);
        assert_eq!(363601, repeating[2]);
        assert_eq!(5, chain.len());
        assert_eq!((vec![69, 363600], vec![1454, 169, 363601]), *chain.get(&69).unwrap());
        assert_eq!((vec![363600], vec![1454, 169, 363601]), *chain.get(&363600).unwrap());
        assert_eq!((vec![], vec![1454, 169, 363601]), *chain.get(&1454).unwrap());
        assert_eq!((vec![], vec![169, 363601, 1454]), *chain.get(&169).unwrap());
        assert_eq!((vec![], vec![363601, 1454, 169]), *chain.get(&363601).unwrap());
    }
}