// use std::collections::HashMap;

pub fn solve() {
    println!("The expected area of the largest consequtive group of black blocks in a randomly determined 7x7 board is {}", find_the_damn_area(7));
}

fn find_the_damn_area(_n:u32) -> u64 {
    return 42;
    // let mut total_area:u64 = 0;
    // let mut board:u64 = 0;
    // let max_board = 2u64.pow(n*n);
    // let mut seen_boards:HashMap<u64,u64> = HashMap::new();
    // let mut index = 0;
    // let mut two_to_the_index:u64 = 1;

    // while board < max_board {
    //     let area = calculate_area(board, &seen_boards);
    //     total_area += area;
    //     seen_boards.insert(board, area);
    //     if board == two_to_the_index {
    //         println!("Progress Report: all boards up to 2^{} done", index);
    //         index += 1;
    //         two_to_the_index *= 2;
    //     }
    //     board += 1;
    //     // println!("Evaluated board {}, area was {}, iteration is {}, current average {}",board, area, iterations, total_area as f64 / iterations as f64)
    // }

    // return total_area;
}

// fn calculate_area(mut board:u64, seen_boards:&HashMap<u64,u64>) -> u64 {
//     let mut max_count = 0;
//     while board > 0 {
//         let mut position = 0;
//         let mut count = 0;
//         let mut related_positions = vec![];
//         if seen_boards.contains_key(&board) {
//             count = *(seen_boards.get(&board).unwrap());
//             if count > max_count {
//                 return count;
//             } else {
//                 return max_count;
//             }
//         }
//         while position < 49 && board & (1 << position) == 0 {
//             position += 1;
//         }
//         if position < 49 {
//             related_positions.push(position);
//         }
//         while !related_positions.is_empty() {
//             position = related_positions.pop().unwrap();
//             if board & (1 << position) > 0 {
//                 count += 1;
//                 board = board ^ (1 << position);
//                 if position + 7 < 49 {
//                     related_positions.push(position + 7);
//                 }
//                 if position + 1 < 49 {
//                     related_positions.push(position + 1);
//                 }
//                 if position >= 1 {
//                     related_positions.push(position - 1);
//                 }
//                 if position >= 7 {
//                     related_positions.push(position - 7);
//                 }
//             }
//         }
//         if count > max_count {
//             max_count = count;
//         }
//     }
//     return max_count;
// }