pub fn solve() {
    let mut max_p = 0;
    let mut max_solution = 0;
    for p in 1..1001 {
        let mut solution = 0;
        for x in 1..p {
            if x > p/2 {
                break;
            }
            for y in 1..p {
                let r = p - x - y;
                if r < 0 {
                    continue;
                }
                if x * x + y * y == r * r {
                    solution += 1;
                }
            }
        }
        if solution > max_solution {
            max_solution = solution;
            max_p = p;
        }
    }
    println!(" num is {}", max_p);
}