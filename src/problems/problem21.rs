use number_theory;

pub fn problem21() {
    let mut amicable_sum = 0;
    let mut vec = vec!();
    vec.push(0);
    for i in 1..10000 {
        let sum = sum_divisors(i);
        vec.push(sum);
        if sum < vec.len() as u64 && vec[sum as usize] == i && vec[i as usize] == sum && i != sum {
            amicable_sum += sum + vec[sum as usize];
        }
    }
    println!("The sum of all amicable numbers under 10,000 is {}", amicable_sum);
}

fn sum_divisors(n: u64) -> u64 {
    return number_theory::number_theory::divisors_include_one(n).iter().fold(0, |a, b| a + b);
}