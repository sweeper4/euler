mod problems;

fn main() {
    println!("sum of the threes and fives is {} lower than {}",problems::problem1(1000),1000);
    println!("total of even fibonacci numbers = {}", problems::problem2(4000000));
    println!("The max prime factor of {} is {}",600851475143_u64,problems::problem3(600851475143));
    println!("Max palindrome {}",problems::problem4(100,999));
    println!("The smallest multiple of numbers less than {} is {}",10,problems::problem5(10));
    println!("The smallest multiple of numbers less than {} is {}",20,problems::problem5(20));
}
