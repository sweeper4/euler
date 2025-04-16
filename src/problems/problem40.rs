/* Ilana's solution:
acc = ""
for i in range(1000000):
    acc += str(i)
print(int(acc[1]) * int(acc[10]) * int(acc[100]) * int(acc[1000]) * int(acc[10000]) * int(acc[100000]) * int(acc[1000000]))
*/

pub fn solve() {
    println!("The product of the 1st, 10th, 100th, 1,000th, 10,000th, 100,000th and 1,000,000th values in Champernowne's constant is {}", mul_powers_of_ten_digits_of_funny_constant());
}

fn mul_powers_of_ten_digits_of_funny_constant() -> usize {
    (0..7).into_iter()
        .map(|x| pow(10,x))
        .map(|x| get_value(x))
        .fold(1, |acc,x| acc * x)
            
}
fn get_value(index: usize) -> usize{
    let decimal_counts: Vec<usize> = (1..7).into_iter().collect();
    let counts: Vec<usize> = decimal_counts.iter().map(|x| pow(10,*x) *9).enumerate()
        .map(|(i,b)| ( i + 1) * b).filter(|x| x < &(index -1)).collect();

    let max_digit_length = counts.len() + 1;
    let pre_calculated = counts.iter().fold(0,|acc,x| acc + x);
    let new_index = (index -1) - pre_calculated;
    let count_nums = new_index / max_digit_length;
    let internal_index = new_index % max_digit_length;
    let base = pow(10, max_digit_length);
    let final_number = (base + count_nums).to_string().chars().nth(internal_index).unwrap();
    match final_number.to_digit(10) {
        Some(a) => a as usize,
        None => 0,
    }
}

fn pow(base: usize, power: usize) -> usize {
    match power {
        0 => 1,
        _ => (1..power).map(|_| base).fold(1, |acc,x| acc * x ),
    }
}
