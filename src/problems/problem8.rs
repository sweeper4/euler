pub fn solve() {
    let very_long_number: String = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string();
    println!("The largest product of {} adjacent values in the very long number is {}", 13, find_largest_product_of_n_adjacent_digits(13, very_long_number));
}

fn find_largest_product_of_n_adjacent_digits(n: u32, number: String) -> i64 {
    let mut max_product: i64 = 1;
    let mut i: u32 = 0;

    let mut chars_front_edge = number.chars();
    let mut chars_back_edge = number.chars();

    while i < n {
        let num: u32 = chars_front_edge.next().unwrap().to_digit(10).unwrap();
        max_product *= num as i64;
        i += 1;
    }

    let mut front_edge = chars_front_edge.next();
    let mut back_edge = chars_back_edge.next();
    let mut product = max_product;

    while front_edge.is_some() {
        let mut front_value = front_edge.unwrap().to_digit(10).unwrap() as i32;
        if front_value == 0 {
            front_value = -1;
        }
        let mut back_value = back_edge.unwrap().to_digit(10).unwrap() as i32;
        if back_value == 0 {
            back_value = -1;
        }
        product = product / back_value as i64 * front_value as i64;
        if product > max_product {
            max_product = product;
        }
        front_edge = chars_front_edge.next();
        back_edge = chars_back_edge.next();
    }

    return max_product;
}