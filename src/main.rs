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
    pub mod problem30;
    pub mod problem31;
    pub mod problem32;
    pub mod problem33;
    pub mod problem34;
    pub mod problem35;
    pub mod problem36;
    pub mod problem37;
    pub mod problem38;
    pub mod problem39;
    pub mod problem40;
    pub mod problem41;
    pub mod problem42;
    pub mod problem43;
    pub mod problem44;
    pub mod problem45;
    pub mod problem46;
    pub mod problem47;
    pub mod problem48;
    pub mod problem49;
    pub mod problem50;
    pub mod problem51;
    pub mod problem52;
    pub mod problem53;
    pub mod problem54;
    pub mod problem55;
    pub mod problem56;
    pub mod problem57;
    pub mod problem58;
    pub mod problem59;
    pub mod problem60;
    pub mod problem61;
    pub mod problem62;
    pub mod problem63;
    pub mod problem64;
    pub mod problem65;
    pub mod problem66;
    pub mod problem67;
    pub mod problem68;
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
        (1, Box::new(problems::problem1::solve as fn() -> ())),
        (2, Box::new(problems::problem2::solve as fn() -> ())),
        (3, Box::new(problems::problem3::solve as fn() -> ())),
        (4, Box::new(problems::problem4::solve as fn() -> ())),
        (5, Box::new(problems::problem5::solve as fn() -> ())),
        (6, Box::new(problems::problem6::solve as fn() -> ())),
        (7, Box::new(problems::problem7::solve as fn() -> ())),
        (8, Box::new(problems::problem8::solve as fn() -> ())),
        (9, Box::new(problems::problem9::solve as fn() -> ())),
        (10, Box::new(problems::problem10::solve as fn() -> ())),
        (11, Box::new(problems::problem11::solve as fn() -> ())),
        (12, Box::new(problems::problem12::solve as fn() -> ())),
        (13, Box::new(problems::problem13::solve as fn() -> ())),
        (14, Box::new(problems::problem14::solve as fn() -> ())),
        (15, Box::new(problems::problem15::solve as fn() -> ())),
        (16, Box::new(problems::problem16::solve as fn() -> ())),
        (17, Box::new(problems::problem17::solve as fn() -> ())),
        (18, Box::new(problems::problem18::solve as fn() -> ())),
        (19, Box::new(problems::problem19::solve as fn() -> ())),
        (20, Box::new(problems::problem20::solve as fn() -> ())),
        (21, Box::new(problems::problem21::solve as fn() -> ())),
        (22, Box::new(problems::problem22::solve as fn() -> ())),
        (23, Box::new(problems::problem23::solve as fn() -> ())),
        (24, Box::new(problems::problem24::solve as fn() -> ())),
        (25, Box::new(problems::problem25::solve as fn() -> ())),
        (26, Box::new(problems::problem26::solve as fn() -> ())),
        (27, Box::new(problems::problem27::solve as fn() -> ())),
        (28, Box::new(problems::problem28::solve as fn() -> ())),
        (29, Box::new(problems::problem29::solve as fn() -> ())),
        (30, Box::new(problems::problem30::solve as fn() -> ())),
        (31, Box::new(problems::problem31::solve as fn() -> ())),
        (32, Box::new(problems::problem32::solve as fn() -> ())),
        (33, Box::new(problems::problem33::solve as fn() -> ())),
        (34, Box::new(problems::problem34::solve as fn() -> ())),
        (35, Box::new(problems::problem35::solve as fn() -> ())),
        (36, Box::new(problems::problem36::solve as fn() -> ())),
        (37, Box::new(problems::problem37::solve as fn() -> ())),
        (38, Box::new(problems::problem38::solve as fn() -> ())),
        (39, Box::new(problems::problem39::solve as fn() -> ())),
        (40, Box::new(problems::problem40::solve as fn() -> ())),
        (41, Box::new(problems::problem41::solve as fn() -> ())),
        (42, Box::new(problems::problem42::solve as fn() -> ())),
        (43, Box::new(problems::problem43::solve as fn() -> ())),
        (44, Box::new(problems::problem44::solve as fn() -> ())),
        (45, Box::new(problems::problem45::solve as fn() -> ())),
        (46, Box::new(problems::problem46::solve as fn() -> ())),
        (47, Box::new(problems::problem47::solve as fn() -> ())),
        (48, Box::new(problems::problem48::solve as fn() -> ())),
        (49, Box::new(problems::problem49::solve as fn() -> ())),
        (50, Box::new(problems::problem50::solve as fn() -> ())),
        (51, Box::new(problems::problem51::solve as fn() -> ())),
        (52, Box::new(problems::problem52::solve as fn() -> ())),
        (53, Box::new(problems::problem53::solve as fn() -> ())),
        (54, Box::new(problems::problem54::solve as fn() -> ())),
        (55, Box::new(problems::problem55::solve as fn() -> ())),
        (56, Box::new(problems::problem56::solve as fn() -> ())),
        (57, Box::new(problems::problem57::solve as fn() -> ())),
        (58, Box::new(problems::problem58::solve as fn() -> ())),
        (59, Box::new(problems::problem59::solve as fn() -> ())),
        (60, Box::new(problems::problem60::solve as fn() -> ())),
        (61, Box::new(problems::problem61::solve as fn() -> ())),
        (62, Box::new(problems::problem62::solve as fn() -> ())),
        (63, Box::new(problems::problem63::solve as fn() -> ())),
        (64, Box::new(problems::problem64::solve as fn() -> ())),
        (65, Box::new(problems::problem65::solve as fn() -> ())),
        (66, Box::new(problems::problem66::solve as fn() -> ())),
        (67, Box::new(problems::problem67::solve as fn() -> ())),
        (68, Box::new(problems::problem68::solve as fn() -> ())),
        (701, Box::new(problems::problem701::solve as fn() -> ()))
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

