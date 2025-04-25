pub fn solve() {
    let mut best_sum = 0;
    for total in 10..20 {
        for o1 in 1_u32..11 {
            for i1 in 1..10 {
                if [o1].contains(&i1) {
                    continue;
                }
                if total < o1 + i1 {
                    continue;
                }
                let i2 = total - o1 - i1;
                if [i1,o1].contains(&i2) || i2 > 10 || i2 == 0 {
                    continue;
                }
                for o2 in o1+1..11 {
                    if [i1,i2].contains(&o2) {
                        continue;
                    }
                    if total < o2 + i2 {
                        continue;
                    }
                    let i3 = total - o2 - i2;
                    if [i1,i2,o1,o2].contains(&i3) || i3 > 10 || i3 == 0 {
                        continue;
                    }
                    for o3 in o2+1..11 {
                        if [i1,i2,i3].contains(&o3) {
                            continue;
                        }
                        if total < o3 + i3 {
                            continue;
                        }
                        let i4 = total - o3 - i3;
                        if [i1,i2,i3,o1,o2,o3].contains(&i4) || i4 > 10 || i4 == 0 {
                            continue;
                        }
                        for o4 in o3+1..11 {
                            if [i1,i2,i3,i4].contains(&o4) {
                                continue;
                            }
                            if total < i4 + o4 {
                                continue;
                            }
                            let i5 = total - i4 - o4;
                            if [i1,i2,i3,i4,o1,o2,o3,o4].contains(&i5) || i5 > 10 || i5 == 0 {
                                continue;
                            }
                            if total < i1 + i5 {
                                continue;
                            }
                            let o5 = total - i1 - i5;
                            if [i1,i2,i3,i4,i5,o1,o2,o3,o4].contains(&o5) || o5 > 10 || o5 == 0 || o5 <= o4 {
                                continue;
                            }
                            let a:String = [o1,i1,i2,o2,i2,i3,o3,i3,i4,o4,i4,i5,o5,i5,i1].iter().map(|x| x.to_string()).collect();
                            if a.len() != 16 {
                                continue;
                            }
                            println!("{:?},{:?},{:?},{:?},{:?}",[o1,i1,i2],[o2,i2,i3],[o3,i3,i4],[o4,i4,i5],[o5,i5,i1]);
                            println!("{}",a);
                            let a = a.parse::<u128>().unwrap();
                            if a > best_sum {
                                best_sum = a;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", best_sum)
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}

//7638359521024646