use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("src/assets/problem22Names.csv").expect("Something went wrong reading the file");
    let mut names: Vec<&str> = contents.split(',').collect();

    names.sort();

    let mut sum = 0;

    for i in 0..names.len() {
        sum += (i as u32 + 1) * sum_word(names[i]);
    }

    println!("The total of all names in the file is {}", sum);
}

fn sum_word(word:&str) -> u32 {
    return word.chars().fold(0, |a, b| a + (b as u32 - 'A' as u32 + 1));
}