
fn multiples_of_3_5(upperLimit:u64) -> u64 {
    let mut x: u64 = 0;
    for i in 0..upperLimit {
        if i%3 ==0{ 
            x += i
        } else if i%5 ==0 {
            x += i
        }
    }
    return x;
    
}
pub fn problem1() {
    println!("sum of the threes and fives is {} lower than {}",multiples_of_3_5(1000),1000);
}