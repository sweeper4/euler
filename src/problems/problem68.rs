pub fn solve() {
    let nums = solve_3(5);
    let mut highest = 0;
    for num in nums {
        if num >= 1_000_000_000_000_000 && num < 10_000_000_000_000_000 {
            if num > highest {
                highest = num;
            }
        }
    }
    println!("{}",highest);
}

fn solve_3(num_of_gon: usize) -> Vec<u64> {
    let mut numbers = vec![];
    for total in 2*num_of_gon as u32 .. 4*num_of_gon as u32 {
        let mut vals: Vec<Option<u32>> = vec![];
        for _ in 0..num_of_gon {
            vals.push(None);
            vals.push(None);
        }
        let mut considered_vals: Vec<Vec<Option<u32>>> = vec![vals];
        for i in 0..num_of_gon {
            let mut new_considered_vals = vec![];
            while let Some(vals) = considered_vals.pop() {
                for j in 0..num_of_gon * 2 {
                    let mut new_vals = vals.clone();
                    new_vals[i] = Some(j as u32 + 1);
                    if validate(&new_vals) && extrapolate(num_of_gon, total, &mut new_vals, i) {
                        new_considered_vals.push(new_vals);
                    }
                }
            }
            considered_vals = new_considered_vals;
        }
        let mut new_considered_vals = vec![];
        while let Some(vals) = considered_vals.pop() {
            let mut vals = vals;
            if validate(&vals) && extrapolate(num_of_gon, total, &mut vals, 0) {
                new_considered_vals.push(vals);
            }
        }
        considered_vals = new_considered_vals;
        for vals in considered_vals {
            let number = get_string(&vals, num_of_gon);
            if !numbers.contains(&number) {
                numbers.push(number);
            }
        }
    }
    return numbers;
}

fn get_string(vals: &Vec<Option<u32>>, num_of_gon: usize) -> u64 {
    let mut triples = vec![];
    for i in 0..num_of_gon {
        match (vals[get_inner_index(i, num_of_gon)], vals[get_inner_index(i+num_of_gon-1, num_of_gon)], vals[get_outer_index(i+num_of_gon-1, num_of_gon)]) {
            (Some(a),Some(b),Some(c)) => {
                triples.push((c,b,a));
            }
            _ => {}
        }
    }
    let mut smallest = num_of_gon as u32 * 2;
    let mut smallest_index = 0;
    for (i,(a,_,_)) in triples.iter().enumerate() {
        if *a < smallest {
            smallest = *a;
            smallest_index = i;
        }
    }
    triples.rotate_left(smallest_index);
    let triple:String = triples.iter().map(|(a,b,c)| a.to_string() + &b.to_string() + &c.to_string()).collect();
    return triple.parse().unwrap();
}

fn extrapolate(num_of_gon: usize, total: u32, vals: &mut Vec<Option<u32>>, n: usize) -> bool {
    match (vals[get_inner_index(n, num_of_gon)], vals[get_inner_index(n+num_of_gon-1, num_of_gon)], vals[get_outer_index(n+num_of_gon-1, num_of_gon)]) {
        (Some(a),Some(b),None) => {
            if a + b >= total {
                return false;
            }
            let c = total - a - b;
            if c > num_of_gon as u32 * 2 {
                return false;
            }
            for d in vals.iter() {
                if d.is_some() && d.unwrap() == c {
                    return false;
                }
            }
            vals[get_outer_index(n+num_of_gon-1, num_of_gon)] = Some(c);
        },
        (Some(a),None,Some(c)) => {
            if a + c >= total {
                return false;
            }
            let b = total - a - c;
            if b > num_of_gon as u32 * 2 {
                return false;
            }
            for d in vals.iter() {
                if d.is_some() && d.unwrap() == b {
                    return false;
                }
            }
            vals[get_inner_index(n+num_of_gon-1, num_of_gon)] = Some(b);
        },
        (None,Some(b),Some(c)) => {
            if b + c >= total {
                return false;
            }
            let a = total - b - c;
            if a > num_of_gon as u32 * 2 {
                return false;
            }
            for d in vals.iter() {
                if d.is_some() && d.unwrap() == a {
                    return false;
                }
            }
            vals[get_inner_index(n, num_of_gon)] = Some(a);
        },
        _ => {}
    }
    return true;
}

fn validate(vals: &Vec<Option<u32>>) -> bool {
    let mut seen = vec![];
    for a in vals {
        match *a {
            Some(a) => {
                if seen.contains(&a) {
                    return false;
                }
                seen.push(a);
            }
            _ => {}
        }
    }
    return true;
}

fn get_inner_index(n: usize, num_of_gon: usize) -> usize {
    return (n + num_of_gon) % num_of_gon;
}

fn get_outer_index(n: usize, num_of_gon: usize) -> usize {
    return (n + num_of_gon) % num_of_gon + num_of_gon;
}

#[cfg(test)]
mod tests {
    use super::{extrapolate, solve, solve_3};

    #[test]
    fn test() {
        solve();
    }

    #[test]
    fn test_extrapolate() {
        let mut vals = vec![Some(1),None,None,Some(3),None,None];
        let ex = extrapolate(3, 6, &mut vals, 0);
        assert!(ex);
        assert_eq!(vals,vec![Some(1),Some(2),None,Some(3),None,None]);
        vals[1] = None;
        let ex = extrapolate(3, 5, &mut vals, 0);
        assert!(!ex);
        assert_eq!(vals,vec![Some(1),None,None,Some(3),None,None]);
        let ex = extrapolate(3, 3, &mut vals, 0);
        assert!(!ex);
        assert_eq!(vals,vec![Some(1),None,None,Some(3),None,None]);
    }

    #[test]
    fn test_3() {
        solve_3(3);
        solve_3(5);
    }
}
