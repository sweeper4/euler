/* Ilana's solution:
acc = ""
for i in range(1000000):
    acc += str(i)
print(int(acc[1]) * int(acc[10]) * int(acc[100]) * int(acc[1000]) * int(acc[10000]) * int(acc[100000]) * int(acc[1000000]))
*/

pub fn problem40() {
    println!("The product of the 1st, 10th, 100th, 1,000th, 10,000th, 100,000th and 1,000,000th values in Champernowne's constant is {}", mul_powers_of_ten_digits_of_funny_constant());
}

fn mul_powers_of_ten_digits_of_funny_constant() -> u32 {
    let funny_constant = (0..1000000).fold("".to_string(), |acc, x| [acc, x.to_string()].join(""));
    return funny_constant.chars().nth(1).unwrap().to_digit(10).unwrap() * funny_constant.chars().nth(10).unwrap().to_digit(10).unwrap()
    * funny_constant.chars().nth(100).unwrap().to_digit(10).unwrap() * funny_constant.chars().nth(1000).unwrap().to_digit(10).unwrap()
    * funny_constant.chars().nth(10000).unwrap().to_digit(10).unwrap() * funny_constant.chars().nth(100000).unwrap().to_digit(10).unwrap()
    * funny_constant.chars().nth(1000000).unwrap().to_digit(10).unwrap();
}