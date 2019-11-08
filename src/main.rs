mod problems;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: {} [problem #]", args[0]);
        return;
    }

    if args.len() == 1 {
        problems::problem1();
        problems::problem2();
        problems::problem3();
        problems::problem4();
        problems::problem5();
        problems::problem6();
        problems::problem7();
        problems::problem8();
        problems::problem9();
    } else {
        match args[1].parse::<u32>().unwrap() {
            1 => problems::problem1(),
            2 => problems::problem2(),
            3 => problems::problem3(),
            4 => problems::problem4(),
            5 => problems::problem5(),
            6 => problems::problem6(),
            7 => problems::problem7(),
            8 => problems::problem8(),
            9 => problems::problem9(),
            _ => println!("Usage: {} [problem #]", args[0])
        }
    }
}
