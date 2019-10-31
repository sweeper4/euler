pub fn compute_difference_between_square_of_sums_and_sum_of_squares(n: u32) -> u32 {
    let sum: u32 = crate::problems::problem1::sum_to(n);
    let square_of_sums: u32 = sum * sum;
    let sum_of_squares: u32 = sum_of_squares(n);
    return square_of_sums - sum_of_squares;
}

fn sum_of_squares(n: u32) -> u32 {
    return n * (n + 1) * (2 * n + 1) / 6;
}