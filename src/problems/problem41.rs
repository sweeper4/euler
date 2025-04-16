pub fn solve() {
    println!("Just another pandigital prime problem, answers {}", solve_the_thing());
}

fn solve_the_thing() -> u64 {
    //do not generate all primes and search them for pandigital numbers. generate pandigital numbers and check if they're prime
    
    let mut digits = vec!(9,8,7,6,5,4,3,2,1);

    while !digits.is_empty() {
        let mut pandigial_numbers:Vec<u64> = number_theory::number_theory::permute(digits.clone()).iter().map(|x| condense_vec_into_num(x)).collect();
        pandigial_numbers.sort();
        for pandigit in pandigial_numbers.iter().rev() {
            if number_theory::number_theory::is_prime(*pandigit) {
                return *pandigit;
            }
        }
        digits.remove(0);
    }
    return 2;
}

fn condense_vec_into_num(vec: &Vec<u64>) -> u64 {
    let mut num = 0;
    for digit in vec {
        num *= 10;
        num += digit;
    }
    return num;
}