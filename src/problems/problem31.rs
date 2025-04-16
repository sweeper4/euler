pub fn solve() {
    let value = [200, 100, 50, 20, 10, 5, 2, 1];
    let count = recurse(&value, 200);
    println!("THere are {} ways to make 2 pounds with coins", count);
}

fn recurse(coins: &[u32], remaining_value: u32) -> u32 {
    if remaining_value == 0 {
        return 1;
    }
    if coins.is_empty() {
        return 0;
    }
    if coins[0] <= remaining_value {
        return recurse(coins, remaining_value - coins[0]) + recurse(&coins[1..], remaining_value);
    } else {
        return recurse(&coins[1..], remaining_value);
    }
}