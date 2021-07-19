use std::fs;
use std::collections::HashMap;
use number_theory::number_theory;

pub fn problem42() {
    let mut triangular_nums:HashMap<u64, bool> = HashMap::new();

    let contents = fs::read_to_string("src/assets/problem42words.csv").expect("Something went wrong reading the file");
    let words: Vec<&str> = contents.split(',').collect();

    let mut count = 0;

    for word in words {
        let sum = convert_word_to_sum(word);
        if !triangular_nums.contains_key(&sum) {
            triangular_nums.insert(sum, number_theory::is_triangular(sum));
        }
        match triangular_nums.get(&sum) {
            Some(true) => count += 1,
            Some(false) => {},
            None => {}
        }
    }

    println!("There are {} triangular numbers in the provided text", count);

    // for (sum, triangular) in triangular_nums {
    //     if !triangular {
    //         println!("{} is not triangular", sum);
    //     }
    // }
}

fn convert_word_to_sum(word: &str) -> u64 {
    let mut sum = 0;
    for character in word.chars() {
        if character < 'A' || character > 'Z' {
            println!("'{}' was weird", character);
        }
        let char_sum = character as i64 - 'A' as i64 + 1;
        sum += char_sum;
    }
    if sum < 0 {
        sum *= -1;
    }
    return sum as u64;
}