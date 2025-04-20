use std::collections::HashMap;

pub fn solve() {
    let mut i = 0;
    let mut cubes = HashMap::<(u32,u32,u32,u32,u32,u32,u32,u32,u32,u32),(u32,u128)>::new();
    // let mut cubes = HashMap::<u32,u32>::new();
    loop {
        let num = i * i * i;
        i += 1;
        let hash = order_invariant_hash(&num);
        if !cubes.contains_key(&hash) {
            cubes.insert(hash, (0, num));
        }
        let (count, min_cube) = cubes.get(&hash).unwrap();
        if *count == 4 {
            println!("{}", min_cube);
            return;
        }
        cubes.insert(hash, (*count+1, *min_cube));
    }
}

fn order_invariant_hash(num: &u128) -> (u32,u32,u32,u32,u32,u32,u32,u32,u32,u32) {
    let mut num = *num;
    let mut digit_count = Vec::new();
    for _ in 0..10 {
        digit_count.push(0);
    }
    while num > 0 {
        digit_count[(num%10) as usize] += 1;
        num /= 10;
    }
    return (digit_count[0],digit_count[1],digit_count[2],digit_count[3],digit_count[4],digit_count[5],digit_count[6],digit_count[7],digit_count[8],digit_count[9])
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}