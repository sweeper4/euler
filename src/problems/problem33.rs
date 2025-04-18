use number_theory::number_theory;

pub fn solve() {
    let mut numerator = 1_u64;
    let mut denominator = 1_u64;
    for i in 1..10_u64 {
        for j in 1..10_u64 {
            for k in 0..10_u64 {
                if !(i == j && i == k) && (i*10 + j) * k == i * (j * 10 + k) {
                    numerator *= i;
                    denominator *= k;
                }
            }
        }
    }

    let factor = number_theory::gcd(&numerator, &denominator);

    println!("The product of the four fractions has a denominator of {}", denominator/factor);
}