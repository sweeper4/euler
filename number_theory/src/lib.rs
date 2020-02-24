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

    #[test]
    fn permute_trivial_vec_works() {
        let mut expected = HashSet::new();
        expected.insert(vec![]);
        assert_eq!(number_theory::permute::<u64>(vec![]),expected);
    }

    #[test]
    fn permute_simple_vec_works() {
        let mut expected = HashSet::new();
        expected.insert(vec![1]);
        assert_eq!(number_theory::permute::<u64>(vec![1]),expected);
        expected.clear();
        expected.insert(vec![1,2,3]);
        expected.insert(vec![1,3,2]);
        expected.insert(vec![2,1,3]);
        expected.insert(vec![2,3,1]);
        expected.insert(vec![3,1,2]);
        expected.insert(vec![3,2,1]);
        assert_eq!(number_theory::permute::<u64>(vec![1, 2, 3]), expected);
    }
}
