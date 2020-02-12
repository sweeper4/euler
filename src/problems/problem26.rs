use number_theory;
use std::convert::TryInto;

pub fn problem26() {
    println!("The number with the longest repeating section of it's decimal representation below 1000 is {}", find_longest_period_of_decimal_representations_below(1000));
}

fn find_longest_period_of_decimal_representations_below(n: u32) -> u32 {
    let mut i = 1;
    let mut max_length = 0;
    let mut max_value = 0;
    while i < n {
        let decimal = number_theory::number_theory::transform_fraction_into_decimal_notation(1, i.try_into().unwrap());
        if decimal.repeating_decimal_part.len() > max_length {
            max_length = decimal.repeating_decimal_part.len();
            max_value = i;
        }
        i += 1;
    }
    return max_value;
}