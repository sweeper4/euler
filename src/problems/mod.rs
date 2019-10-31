mod problem1;

pub fn problem1(n: u32) -> u32 {
    return problem1::multiples_of_3_5(n)
}

mod problem2;

pub fn problem2(n: usize) -> usize {
    return problem2::sum_evens_less_than(n)
}

mod problem3;

pub fn problem3(n: usize) -> usize {
    return problem3::get_max_factor(n)
}

mod problem4;

pub fn problem4(min:usize, max:usize) -> usize {
    return problem4::max_palindrome_multiple_between(min, max)
}

mod problem5;

pub fn problem5(max:usize) -> usize {
    return problem5::smallest_multiple(max)
}

mod problem6;

pub fn problem6(n: u32) -> u32 {
    return problem6::compute_difference_between_square_of_sums_and_sum_of_squares(n)
}