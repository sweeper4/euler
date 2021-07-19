fn compute_difference_between_square_of_sums_and_sum_of_squares(n: u32) -> u32 {
    let sum: u32 = sum_to(n);
    let square_of_sums: u32 = sum * sum;
    let sum_of_squares: u32 = sum_of_squares(n);
    return square_of_sums - sum_of_squares;
}

fn sum_of_squares(n: u32) -> u32 {
    return n * (n + 1) * (2 * n + 1) / 6;
}

fn sum_to(max: u32) -> u32 {
    (max * (max + 1)) / 2
}

pub fn problem6() {
    println!("The difference between the sum of the squares and the square of the sum from 1 to 100 is {}",compute_difference_between_square_of_sums_and_sum_of_squares(100));
}