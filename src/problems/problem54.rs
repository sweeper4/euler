use std::collections::{HashMap, HashSet};


pub fn solve() {
    
}

fn score_hand(hand: Vec<(u8, Suit)>) -> (u8,u64) {
    let mut royal = HashSet::new();
    royal.insert(10);
    royal.insert(11);
    royal.insert(12);
    royal.insert(13);
    royal.insert(14);
    let mut suits = HashSet::new();
    let mut ranks: HashMap<u8, u64> = HashMap::new();
    for (rank, suit) in hand {
        if !ranks.contains_key(&rank) {
            ranks.insert(rank, 1);
        } else {
            ranks.insert(rank, ranks.get(&rank).unwrap()+1);
        }
        if !suits.contains(&suit) {
            suits.insert(suit);
        }
    }
    if ranks.len() == 5 {
        if find_straight(&ranks) {
            if find_flush(&suits) {
                if ranks.contains_key(&14) {
                    // royal flush
                    return (10, 0);
                } else {
                    return (9, simple_score(&ranks));
                }
            } else {
                // straight
                return (5, simple_score(&ranks));
            }
        } else if find_flush(&suits) {
            // flush
            return (6, simple_score(&ranks));
        } else {
            // high card
            return (1, simple_score(&ranks));
        }
    } else if ranks.len() == 4 {
        // one pair
        let mut pair_rank = 0;
        for (rank, count) in &ranks {
            if *count == 2 {
                pair_rank = *rank as u64;
            }
        }
        return (2, 10000000000 * pair_rank + simple_score(&ranks))
    } else if ranks.len() == 3 {
        if ranks.iter().any(|(_, count)| *count == 3) {
            // three of a kind
            let mut three = 0;
            for (rank, count) in &ranks {
                if *count == 3 {
                    three = *rank as u64;
                }
            }
            return (4, 10000000000 * three + simple_score(&ranks));
        } else {
            // two pair
            let mut pair_one = 0;
            let mut pair_two = 0;
            for (rank, count) in &ranks {
                if *count == 2 && pair_one == 0 {
                    pair_one = *rank as u64;
                } else if *count == 2 {
                    pair_two = *rank as u64;
                }
            }
            if pair_one < pair_two {
                (pair_one, pair_two) = (pair_two, pair_one);
            }
            return (3, 1000000000000 * pair_one + 10000000000 * pair_two + simple_score(&ranks));
        }
    } else if ranks.len() == 2 {
        if ranks.iter().any(|(_, count)| *count == 4) {
            // four of a kind
            let mut four = 0;
            let mut one = 0;
            for (rank, count) in ranks {
                if count == 4 {
                    four = rank as u64;
                } else {
                    one = rank as u64;
                }
            }
            return (8, 100 * four + one);
        } else {
            // full house
            let mut three = 0;
            let mut two = 0;
            for (rank, count) in ranks {
                if count == 3 {
                    three = rank as u64;
                } else {
                    two = rank as u64;
                }
            }
            return (8, 100 * three + two);
        }
        
    } else {
        // something broke
    }
    return (0,0);
}

fn find_straight(ranks: &HashMap<u8,u64>) -> bool {
    for i in 2..10 {
        let mut found_straight = true;
        for j in 0..5 {
            if !ranks.contains_key(&(i+j)) {
                found_straight = false;
            }
        }
        if found_straight {
            return true;
        }
    }
    false
}

fn find_flush(suits: &HashSet<Suit>) -> bool {
    return suits.len() == 1;
}

fn simple_score(ranks: &HashMap<u8, u64>) -> u64 {
    let mut ordered_ranks = vec![];
    for rank in ranks.keys() {
        ordered_ranks.push(*rank);
    }
    ordered_ranks.sort();
    return ordered_ranks.iter().fold(0, |init, rank| init * 100 + (*rank as u64));
}

#[derive(Hash, Eq, PartialEq)]
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}