mod problems {
    pub mod problem1;
    pub mod problem2;
    pub mod problem3;
    pub mod problem4;
    pub mod problem5;
    pub mod problem6;
    pub mod problem7;
    pub mod problem8;
    pub mod problem9;
    pub mod problem10;
    pub mod problem11;
    pub mod problem12;
    pub mod problem13;
    pub mod problem14;
    pub mod problem17;
    pub mod problem20;
}

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: {} [problem #]", args[0]);
        return;
    }

    if args.len() == 1 {
        problems::problem1::problem1();
        problems::problem2::problem2();
        problems::problem3::problem3();
        problems::problem4::problem4();
        problems::problem5::problem5();
        problems::problem6::problem6();
        problems::problem7::problem7();
        problems::problem8::problem8();
        problems::problem9::problem9();
        problems::problem10::problem10();
        problems::problem11::problem11();
        problems::problem12::problem12();
        problems::problem13::problem13();
        problems::problem14::problem14();
        problems::problem17::problem17();
        problems::problem20::problem20();
    } else {
        match args[1].parse::<u32>().unwrap() {
            1 => problems::problem1::problem1(),
            2 => problems::problem2::problem2(),
            3 => problems::problem3::problem3(),
            4 => problems::problem4::problem4(),
            5 => problems::problem5::problem5(),
            6 => problems::problem6::problem6(),
            7 => problems::problem7::problem7(),
            8 => problems::problem8::problem8(),
            9 => problems::problem9::problem9(),
            10 => problems::problem10::problem10(),
            11 => problems::problem11::problem11(),
            12 => problems::problem12::problem12(),
            13 => problems::problem13::problem13(),
            14 => problems::problem14::problem14(),
            17 => problems::problem17::problem17(),
            20 => problems::problem20::problem20(),
            _ => println!("Usage: {} [problem #]", args[0])
        }
    }
}
