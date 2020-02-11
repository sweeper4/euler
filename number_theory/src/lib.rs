#[allow(unused_imports)]
use std::collections::HashSet;
pub mod number_theory;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors_include_one_and_n_works() {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(5);
        assert_eq!(number_theory::divisors_include_one_and_n(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        set.insert(8);
        assert_eq!(number_theory::divisors_include_one_and_n(8), set);
    }

    #[test]
    fn divisors_include_one_works() {
        let mut set = HashSet::new();
        set.insert(1);
        assert_eq!(number_theory::divisors_include_one(5), set);
        set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(4);
        assert_eq!(number_theory::divisors_include_one(8), set);
    }
}
