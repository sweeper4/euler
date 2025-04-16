use number_theory;

pub fn solve() {
    println!("There are {} down/right ways through a 20x20 grid", get_routes_through_grid(20,20));
}

fn get_routes_through_grid(n:u128,m:u128) -> u128 {
    return number_theory::number_theory::a_choose_b(n+m,n);
}