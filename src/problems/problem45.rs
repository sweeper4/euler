pub fn solve() {
    let mut pent_index = 166;
    let mut hex_index = 144;

    loop {
        if pent(pent_index) == hex(hex_index) {
            println!("The next tri/penta/hexa number is {}", pent(pent_index));
            return;
        }
        if pent(pent_index) > hex(hex_index) {
            hex_index += 1;
        } else {
            pent_index += 1;
        }
    }
}

fn pent(n: u64) -> u64 {
    return n * (3 * n - 1) / 2;
}

fn hex(n: u64) -> u64 {
    return n * (2 * n - 1);
}