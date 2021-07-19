pub fn problem9() {
    println!("The only pythagorean triple that sums to 1000 has a product of {}", get_pythagorean_triple_that_sums_to(1000));
}

fn get_pythagorean_triple_that_sums_to(n: u32) -> u32 {
    let mut a = 2;
    let mut b = 1;
    while a < n {
        while b < a {
            if a*a + b*b == (n - a - b) * (n - a - b) {
                return a * b * (n - a - b);
            }

            b += 1;
        }

        a += 1;
        b = 1;
    }

    return 0;
}