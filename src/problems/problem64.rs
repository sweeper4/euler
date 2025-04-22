use num::integer::Roots;
use number_theory::number_theory::continued_fraction_of_sqrt;

pub fn solve() {
    let mut count = 0;
    for i in 1..10000 {
        if i.sqrt() * i.sqrt() == i {
            continue;
        }
        let (_,b) = continued_fraction_of_sqrt(i);
        if b.len() % 2 == 1 {
            count += 1;
        }
    }
    println!("{}", count);
}


#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        solve();
    }
}