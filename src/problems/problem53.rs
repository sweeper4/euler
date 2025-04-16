use number_theory::number_theory;

pub fn solve() {
    let mut count = 0;
    for n in 1..101 {
        for r in 0..n/2 {
            let comb = number_theory::a_choose_b(n,r);
            if comb > 1_000_000 {
                if n % 2 == 0 {
                    count += (n/2-r)*2+1;
                    break;
                } else {
                    count += (n/2-r)*2+2;
                    break;
                }
            }
        }
    }
    println!("{}",count);
}


// 11 and 2 => 2,3,4,5,6,7,8,9