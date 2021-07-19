pub fn problem30() {
    let mut i = 10;
    let mut running_sum = 0;
    loop {
        let sum = sum_number(i);
        if i == sum {
            running_sum += sum;
        }
        if i > 1000000 {
            break;
        }
        i += 1;
    }
    println!("{} is the sum of all numbers equal to the sum of the fifth powers of their digits", running_sum);
}

fn sum_number(mut n:u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(5);
        n /= 10;
    }
    return sum;
}