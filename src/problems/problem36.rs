pub fn problem36() {
    let mut sum = 0;
    for i in 1..1000000 {
        if is_palindrome(format!("{:b}", i)) && is_palindrome(i.to_string()) {
            sum += i;
        }
    }
    println!("The sum of all decimal/binary palindromes is {}", sum);
}

fn is_palindrome(a: String) -> bool {
    for (i,j) in a.chars().zip(a.chars().rev()) {
        if i != j {
            return false;
        }
    }
    return true;
}