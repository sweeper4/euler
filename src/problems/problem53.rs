use fraction::fraction;

pub fn problem53() {
    println!("There are {} values over 1,000,000 in n choose r where n <= 100", count_combinatorials_over(1000000,101));
}

fn count_combinatorials_over(upper_limit:u64, max_n:u64) -> u64 {
    let mut count = 0;
    for n in 1..max_n {
        for i in 1..(n/2) {
            if combinatorial_over(n,i,upper_limit) {
                count += 2 * (n/2-i + n%2) + (n+1)%2;
                break;
            }
        }
    }
    return count;
}

fn combinatorial_over(n:u64, r:u64, upper_limit:u64) -> bool {
    if r > n-r {
        return combinatorial_over(n,n-r, upper_limit);
    }
    let upper_limit_frac = fraction::Fraction::new(false, upper_limit, 1);
    let mut running_fraction = fraction::Fraction::new(false, 1, 1);
    let mut current_numerator = n;
    let mut current_denumerator = r;
    while current_denumerator > 0 {
        running_fraction *= fraction::Fraction::new(false, current_numerator, current_denumerator);
        if running_fraction > upper_limit_frac {
            return true;
        }
        current_numerator -= 1;
        current_denumerator -= 1;
    }
    return false;
}