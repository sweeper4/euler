pub fn multiples_of_3_5(max : u32) -> u32 {
    let correct = max - 1;
    let sum_threes = 3 * sum_to(correct/3);
    let sum_fives = 5 * sum_to(correct/5);
    let sum_fifteens = 15 * sum_to(correct/15);
    return sum_threes + sum_fives - sum_fifteens
}

fn sum_to(max: u32) -> u32 {
    (max * (max + 1)) / 2
}