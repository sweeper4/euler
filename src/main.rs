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
    pub mod problem15;
    pub mod problem16;
    pub mod problem17;
    pub mod problem18;
    pub mod problem19;
    pub mod problem20;
    pub mod problem21;
    pub mod problem22;
    pub mod problem23;
    pub mod problem24;
    pub mod problem25;
    pub mod problem26;
    pub mod problem27;
    pub mod problem28;
    pub mod problem29;
    pub mod problem32;
    pub mod problem34;
    pub mod problem35;
    pub mod problem38;
    pub mod problem40;
    pub mod problem41;
    pub mod problem44;
    pub mod problem52;
    pub mod problem701;
}
extern crate threadpool;

use std::env;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    let workers = 12;
    let pool = ThreadPool::new(workers);

    if args.len() > 2 {
        println!("Usage: {} [problem #]", args[0]);
        return;
    }
    let problems : HashMap<u32,Box<fn() -> ()>> = [
        (1, Box::new(problems::problem1::problem1 as fn() -> ())),
        (2, Box::new(problems::problem2::problem2 as fn() -> ())),
        (3, Box::new(problems::problem3::problem3 as fn() -> ())),
        (4, Box::new(problems::problem4::problem4 as fn() -> ())),
        (5, Box::new(problems::problem5::problem5 as fn() -> ())),
        (6, Box::new(problems::problem6::problem6 as fn() -> ())),
        (7, Box::new(problems::problem7::problem7 as fn() -> ())),
        (8, Box::new(problems::problem8::problem8 as fn() -> ())),
        (9, Box::new(problems::problem9::problem9 as fn() -> ())),
        (10, Box::new(problems::problem10::problem10 as fn() -> ())),
        (11, Box::new(problems::problem11::problem11 as fn() -> ())),
        (12, Box::new(problems::problem12::problem12 as fn() -> ())),
        (13, Box::new(problems::problem13::problem13 as fn() -> ())),
        (14, Box::new(problems::problem14::problem14 as fn() -> ())),
        (15, Box::new(problems::problem15::problem15 as fn() -> ())),
        (16, Box::new(problems::problem16::problem16 as fn() -> ())),
        (17, Box::new(problems::problem17::problem17 as fn() -> ())),
        (18, Box::new(problems::problem18::problem18 as fn() -> ())),
        (19, Box::new(problems::problem19::problem19 as fn() -> ())),
        (20, Box::new(problems::problem20::problem20 as fn() -> ())),
        (21, Box::new(problems::problem21::problem21 as fn() -> ())),
        (22, Box::new(problems::problem22::problem22 as fn() -> ())),
        (23, Box::new(problems::problem23::problem23 as fn() -> ())),
        (24, Box::new(problems::problem24::problem24 as fn() -> ())),
        (25, Box::new(problems::problem25::problem25 as fn() -> ())),
        (26, Box::new(problems::problem26::problem26 as fn() -> ())),
        (27, Box::new(problems::problem27::problem27 as fn() -> ())),
        (28, Box::new(problems::problem28::problem28 as fn() -> ())),
        (29, Box::new(problems::problem29::problem29 as fn() -> ())),
        (32, Box::new(problems::problem32::problem32 as fn() -> ())),
        (34, Box::new(problems::problem34::problem34 as fn() -> ())),
        (35, Box::new(problems::problem35::problem35 as fn() -> ())),
        (38, Box::new(problems::problem38::problem38 as fn() -> ())),
        (40, Box::new(problems::problem40::problem40 as fn() -> ())),
        (41, Box::new(problems::problem41::problem41 as fn() -> ())),
        (44, Box::new(problems::problem44::problem44 as fn() -> ())),
        (52, Box::new(problems::problem52::problem52 as fn() -> ())),
        (701, Box::new(problems::problem701::problem701 as fn() -> ()))
    ].iter().cloned().collect();
    let job_count = problems.len();

    if args.len() == 1 {
        let (tx, rx) = channel();
        for (_, problem) in problems {
            let tx = tx.clone();
            pool.execute( move ||{
                (*problem)();
                tx.send(1).expect("send");
            });
        }
        rx.iter().take(job_count).fold(0, |a,b| a+ b);
    } else {
        let problem_num = args[1].parse::<u32>().unwrap();
        match problems.get(&problem_num) {
            Some(func) => func(),
            None => println!("Usage: {} [problem #]", args[0])
        }
    }
}

