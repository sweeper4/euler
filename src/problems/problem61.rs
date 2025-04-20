pub fn solve() {
    let s1: Vec<(i32, u64)> = get_triangular().iter().map(|x| (1,*x)).collect();
    let mut s2: Vec<(i32, u64)> = get_square().iter().map(|x| (2,*x)).collect();
    let mut s3: Vec<(i32, u64)> = get_pentagonal().iter().map(|x| (3,*x)).collect();
    let mut s4: Vec<(i32, u64)> = get_hexagonal().iter().map(|x| (4,*x)).collect();
    let mut s5: Vec<(i32, u64)> = get_heptagonal().iter().map(|x| (5,*x)).collect();
    let mut s6: Vec<(i32, u64)> = get_octagonal().iter().map(|x| (6,*x)).collect();

    let mut singles = s1;
    singles.append(&mut s2);
    singles.append(&mut s3);
    singles.append(&mut s4);
    singles.append(&mut s5);
    singles.append(&mut s6);

    let mut pairs = vec![];

    for (order, val) in &singles {
        for (order2, val2) in &singles {
            if *order == *order2 {
                continue;
            }
            if *val % 100 == *val2 / 100 {
                pairs.push((*order, *val, *order2, *val2));
            }
        }
    }

    let mut quads = vec![];

    for (o1, v1, o2, v2) in &pairs {
        for (o3, v3, o4, v4) in &pairs {
            if o1 == o3 || o1 == o4 || o2 == o3 || o2 == o4 {
                continue;
            }
            if *v2 % 100 == *v3 / 100 {
                quads.push((*o1,*v1,*o2,*v2,*o3,*v3,*o4,*v4));
            }
        }
    }

    let mut hexes = vec![];

    for (o1,v1,o2,v2,o3,v3,o4,v4) in &quads {
        for (o5,v5,o6,v6) in &pairs {
            if o1 == o5 || o1 == o6 || o2 == o5 || o2 == o6 || o3 == o5 || o3 == o6 || o4 == o5 || o4 == o6 {
                continue;
            }
            if *v4 % 100 == *v5 / 100 && *v6 % 100 == *v1 / 100 {
                hexes.push((*o1,*v1,*o2,*v2,*o3,*v3,*o4,*v4,*o5,*v5,*o6,*v6));
            }
        }
    }

    for hex in hexes {
        println!("{:?}, {}", hex, hex.1 + hex.3 + hex.5 + hex.7 + hex.9 + hex.11);
    }
}

fn get_triangular() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * (i+1) / 2;
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

fn get_square() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * i;
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

fn get_pentagonal() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * (3*i - 1) / 2;
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

fn get_hexagonal() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * (2*i - 1);
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

fn get_heptagonal() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * (5*i - 3) / 2;
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

fn get_octagonal() -> Vec<u64> {
    let mut i = 1;
    let mut values = vec![];
    loop {
        let val = i * (3*i - 2);
        i += 1;
        if val < 1000 {
            continue;
        }
        if val > 9999 {
            return values;
        }
        values.push(val);
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}