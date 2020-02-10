use std::collections::HashMap;

pub fn problem17() {
    println!("If all the numbers 1 to 1000 were written out, {} letters would be used", count_characters_in_all_numbers_less_than(1000));
}

fn count_characters_in_all_numbers_less_than(n:u32) -> u32 {
    let mut number_to_char_count: HashMap<u32,u32> = HashMap::new();
    number_to_char_count.insert(1,3);
    number_to_char_count.insert(2,3);
    number_to_char_count.insert(3,5);
    number_to_char_count.insert(4,4);
    number_to_char_count.insert(5,4);
    number_to_char_count.insert(6,3);
    number_to_char_count.insert(7,5);
    number_to_char_count.insert(8,5);
    number_to_char_count.insert(9,4);
    number_to_char_count.insert(10,3);
    number_to_char_count.insert(11,6);
    number_to_char_count.insert(12,6);
    number_to_char_count.insert(13,8);
    number_to_char_count.insert(14,8);
    number_to_char_count.insert(15,7);
    number_to_char_count.insert(16,7);
    number_to_char_count.insert(17,9);
    number_to_char_count.insert(18,8);
    number_to_char_count.insert(19,8);
    number_to_char_count.insert(20,6);
    number_to_char_count.insert(30,6);
    number_to_char_count.insert(40,5);
    number_to_char_count.insert(50,5);
    number_to_char_count.insert(60,5);
    number_to_char_count.insert(70,7);
    number_to_char_count.insert(80,6);
    number_to_char_count.insert(90,6);

    let mut i = 1;
    let mut sum = 0;
    while i <= n {
        let count = count_characters_in_n(i, &number_to_char_count);
        sum += count;
        i += 1;
    }

    return sum;
}

fn count_characters_in_n(n: u32, number_to_char_count: &HashMap<u32,u32>) -> u32 {
    let hundred:u32 = 7;
    let thousand:u32 = 8;

    let mut count = 0;
    match number_to_char_count.get(&n) {
        Some(value) => count += *value,
        None => if n < 100 {
                count += number_to_char_count.get(&(n - (n%10))).unwrap() + number_to_char_count.get(&(n%10)).unwrap();
            } else if n < 1000 {
                count += count_characters_in_n(n/100, number_to_char_count) + hundred;
                if n % 100 != 0 {
                    count += 3 + count_characters_in_n(n%100, number_to_char_count);
                }
            } else {
                count += count_characters_in_n(n/1000, number_to_char_count) + thousand;
            }
    }
    return count;
}