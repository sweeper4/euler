use std::char;

pub fn problem24() {
    let mut index = 0;
    let goal = 1000000;
    let mut digits = vec!(0,1,2,3,4,5,6,7,8,9);
    let mut ordered_digits = String::from("");
    
    for i in 0..9 {
        let a = (goal - index) / factorial(9-i);
        ordered_digits.push(char::from_digit(digits[a as usize], 10).unwrap());
        digits.remove(a as usize);
        index += a * factorial(9-i);
    }

    println!("the millionth lexicographic ordering of 0-9 is {}", ordered_digits);
}

fn factorial(mut n:u32) -> u32 {
    let mut product = 1;
    while n > 1 {
        product *= n;
        n -= 1;
    }
    return product;
}