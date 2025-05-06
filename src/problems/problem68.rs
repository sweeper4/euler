use std::collections::HashSet;

pub fn solve() {
    let mut best_sum = 0;
    for total in 10..20 {
        for o1 in 1_u32..11 {
            for i1 in 1..11 {
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
                for o2 in 1..11 {
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
                    for o3 in 1..11 {
                        if [i1,i2,i3,o1,o2].contains(&o3) {
                            continue;
                        }
                        if total < o3 + i3 {
                            continue;
                        }
                        let i4 = total - o3 - i3;
                        if [i1,i2,i3,o1,o2,o3].contains(&i4) || i4 > 10 || i4 == 0 {
                            continue;
                        }
                        for o4 in 1..11 {
                            if [i1,i2,i3,i4,o1,o2,o3].contains(&o4) {
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

fn solve_2(num_of_gon: usize) {
    let mut innies = vec![];
    let mut outies = vec![];
    for _ in 0..num_of_gon {
        innies.push(0);
        outies.push(0);
    }
    let mut biggest_num = 0;
    for triple_sum in (3*num_of_gon)..(5*num_of_gon) {
        let arrangements = solve_2_rec(&mut innies, &mut outies, 0, num_of_gon, triple_sum);
        if biggest_num > 0 && arrangements.len() == 0 {
            break;
        }
        for arrangement in arrangements {
            let num:String = arrangement.iter().map(|(a,b,c)| (*a).to_string() + &(*b).to_string() + &(*c).to_string()).collect();
            if num.len() != 16 {
                continue;
            }
            println!("{:?}", arrangement);
            let num = num.parse::<u64>().unwrap();
            if num > biggest_num {
                biggest_num = num;
            }
        }
    }
    println!("{}",biggest_num);
}

fn solve_2_rec(innies: &mut Vec<usize>, outies: &mut Vec<usize>, index: usize, num_of_gon: usize, target: usize) -> HashSet<Vec<(usize,usize,usize)>> {
    let mut solutions = HashSet::new();
    for i in 1..(2*num_of_gon + 1) {
        innies[index] = 0;
        outies[index] = 0;
        if innies.contains(&i) || outies.contains(&i) {
            continue;
        }
        innies[index] = i;
        if index > 0 {
            if target <= innies[index] + innies[index - 1] {
                continue;
            }
            let outie = target - innies[index] - innies[index - 1];
            if outies.contains(&outie) || innies.contains(&outie) || outie > 2 * num_of_gon {
                continue;
            }
            outies[index] = outie;
        }
        if index + 1 == num_of_gon {
            if target <= innies[0] + innies[num_of_gon - 1] {
                continue;
            }
            let outie = target - innies[0] - innies[num_of_gon - 1];
            if outies.contains(&outie) || innies.contains(&outie) || outie > 2 * num_of_gon {
                continue;
            }
            outies[0] = outie;
            let mut solution = vec![];
            for i in 0..num_of_gon {
                solution.push((outies[i], innies[(i+num_of_gon-1) % num_of_gon], innies[i]));
            }
            let mut lowest_index = 0;
            let mut lowest = 2*num_of_gon;
            for (i,(a,_,_)) in solution.iter().enumerate() {
                if *a < lowest {
                    lowest = *a;
                    lowest_index = i;
                }
            }
            solution.rotate_left(lowest_index);
            solutions.insert(solution);
        } else {
            let sols = solve_2_rec(innies, outies, index + 1, num_of_gon, target);
            solutions = &solutions | &sols;
        }
    }
    innies[index] = 0;
    outies[index] = 0;
    return solutions;
}

#[cfg(test)]
mod tests {
    use super::{solve, solve_2};

    #[test]
    fn test() {
        solve();
    }

    #[test]
    fn test_2() {
        solve_2(3);
    }
}

//7638359521024646
//2951051817673439