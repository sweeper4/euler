pub fn solve() {
    let mut i = 1;
    let mut sum = 1;
    let mut increase = 2;
    loop {
        i += increase;
        sum += i;
        i += increase;
        sum += i;
        i += increase;
        sum += i;
        i += increase;
        sum += i;
        increase += 2;
        if i >= 1001 * 1001 {
            break;
        }
    }
    println!("The sum of the diagonals of a 1001 * 1001 square is {}", sum);
}