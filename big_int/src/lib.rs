pub mod big_int;

#[allow(unused_imports)]
pub use big_int::BigInt;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_big_ints_work() {
        assert_eq!(big_int::BigInt::new(5).show_formatted(), "BigInt { 5 }");
        assert_eq!(big_int::BigInt::new(5).show(), "5");
    }
    
    #[test]
    fn simple_negative_big_ints_work() {
        assert_eq!(big_int::BigInt::new(-5).show_formatted(), "BigInt { -5 }");
        assert_eq!(big_int::BigInt::new(-5).show(), "-5");
    }
    
    #[test]
    fn positive_less_than_positive_works() {
        assert_eq!(big_int::BigInt::new(12345) < big_int::BigInt::new(67890), true);
    }
    
    #[test]
    fn positive_equal_to_positive_works() {
        assert_eq!(big_int::BigInt::new(12345) == big_int::BigInt::new(12345), true);
    }
    
    #[test]
    fn positive_greater_than_positive_works() {
        assert_eq!(big_int::BigInt::new(12345) > big_int::BigInt::new(1234), true);
    }
    
    #[test]
    fn positive_greater_than_negative_with_less_abs_works() {
        assert_eq!(big_int::BigInt::new(12345) > big_int::BigInt::new(-1234), true);
    }
    
    #[test]
    fn positive_greater_than_negative_with_equal_abs_works() {
        assert_eq!(big_int::BigInt::new(12345) > big_int::BigInt::new(-12345), true);
    }
    
    #[test]
    fn positive_greater_than_negative_with_greater_abs_works() {
        assert_eq!(big_int::BigInt::new(12345) > big_int::BigInt::new(-67890), true);
    }
    
    #[test]
    fn negative_less_than_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345) < big_int::BigInt::new(-6789), true);
    }
    
    #[test]
    fn negative_equal_to_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345) == big_int::BigInt::new(-12345), true);
    }
    
    #[test]
    fn negative_greater_than_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345) > big_int::BigInt::new(-67890), true);
    }
    
    #[test]
    fn negative_less_than_positive_with_less_abs_works() {
        assert_eq!(big_int::BigInt::new(-12345) < big_int::BigInt::new(1234), true);
    }
    
    #[test]
    fn negative_less_than_positive_with_equal_abs_works() {
        assert_eq!(big_int::BigInt::new(-12345) < big_int::BigInt::new(12345), true);
    }
    
    #[test]
    fn negative_less_than_positive_with_greater_abs_works() {
        assert_eq!(big_int::BigInt::new(-12345) < big_int::BigInt::new(67890), true);
    }
    
    #[test]
    fn zero_less_than_positive_works() {
        assert_eq!(big_int::BigInt::new(0) < big_int::BigInt::new(67890), true);
    }
    
    #[test]
    fn zero_equal_to_zero_works() {
        assert_eq!(big_int::BigInt::new(0) == big_int::BigInt::new(0), true);
    }
    
    #[test]
    fn zero_greater_than_negative_works() {
        assert_eq!(big_int::BigInt::new(0) > big_int::BigInt::new(-67890), true);
    }
    
    #[test]
    fn positive_plus_positive_works() {
        assert_eq!(big_int::BigInt::new(12345).add(big_int::BigInt::new(67890)), big_int::BigInt::new(80235));
    }
    
    #[test]
    fn positive_plus_negative_result_positive_works() {
        assert_eq!(big_int::BigInt::new(12345).add(big_int::BigInt::new(-6789)), big_int::BigInt::new(12345-6789));
    }
    
    #[test]
    fn positive_plus_negative_result_zero_works() {
        assert_eq!(big_int::BigInt::new(12345).add(big_int::BigInt::new(-12345)), big_int::BigInt::new(0));
    }
    
    #[test]
    fn positive_plus_negative_result_negative_works() {
        assert_eq!(big_int::BigInt::new(12345).add(big_int::BigInt::new(-67890)), big_int::BigInt::new(12345-67890));
    }
    
    #[test]
    fn negative_plus_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345).add(big_int::BigInt::new(-67890)), big_int::BigInt::new(-80235));
    }
    
    #[test]
    fn negative_plus_positive_result_positive_works() {
        assert_eq!(big_int::BigInt::new(-12345).add(big_int::BigInt::new(67890)), big_int::BigInt::new(67890-12345));
    }
    
    #[test]
    fn negative_plus_positive_result_zero_works() {
        assert_eq!(big_int::BigInt::new(-12345).add(big_int::BigInt::new(12345)), big_int::BigInt::new(0));
    }
    
    #[test]
    fn negative_plus_positive_result_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345).add(big_int::BigInt::new(6789)), big_int::BigInt::new(-12345+6789));
    }

    #[test]
    fn positive_minus_positive_result_positive_works() {
        assert_eq!(big_int::BigInt::new(12345).sub(big_int::BigInt::new(6789)), big_int::BigInt::new(12345-6789))
    }

    #[test]
    fn positive_minus_positive_result_zero_works() {
        assert_eq!(big_int::BigInt::new(12345).sub(big_int::BigInt::new(12345)), big_int::BigInt::new(0))
    }

    #[test]
    fn positive_minus_positive_result_negative_works() {
        assert_eq!(big_int::BigInt::new(12345).sub(big_int::BigInt::new(67890)), big_int::BigInt::new(12345-67890))
    }

    #[test]
    fn positive_minus_negative_works() {
        assert_eq!(big_int::BigInt::new(12345).sub(big_int::BigInt::new(-6789)), big_int::BigInt::new(12345+6789))
    }

    #[test]
    fn negative_minus_negative_result_positive_works() {
        assert_eq!(big_int::BigInt::new(-12345).sub(big_int::BigInt::new(-67890)), big_int::BigInt::new(-12345+67890))
    }

    #[test]
    fn negative_minus_negative_result_zero_works() {
        assert_eq!(big_int::BigInt::new(-12345).sub(big_int::BigInt::new(-12345)), big_int::BigInt::new(0))
    }

    #[test]
    fn negative_minus_negative_result_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345).sub(big_int::BigInt::new(-6789)), big_int::BigInt::new(6789-12345))
    }

    #[test]
    fn negative_minus_positive_works() {
        assert_eq!(big_int::BigInt::new(-12345).sub(big_int::BigInt::new(6789)), big_int::BigInt::new(-12345-6789))
    }

    #[test]
    fn positive_times_positive_works() {
        assert_eq!(big_int::BigInt::new(12345).mul(big_int::BigInt::new(6789)),big_int::BigInt::new(12345*6789));
    }

    #[test]
    fn positive_times_zero_works() {
        assert_eq!(big_int::BigInt::new(12345).mul(big_int::BigInt::new(0)),big_int::BigInt::new(0));
    }

    #[test]
    fn positive_times_negative_works() {
        assert_eq!(big_int::BigInt::new(12345).mul(big_int::BigInt::new(-6789)),big_int::BigInt::new(12345*-6789));
    }

    #[test]
    fn zero_times_positive_works() {
        assert_eq!(big_int::BigInt::new(0).mul(big_int::BigInt::new(6789)),big_int::BigInt::new(0));
    }

    #[test]
    fn zero_times_zero_works() {
        assert_eq!(big_int::BigInt::new(0).mul(big_int::BigInt::new(0)),big_int::BigInt::new(0));
    }

    #[test]
    fn zero_times_negative_works() {
        assert_eq!(big_int::BigInt::new(0).mul(big_int::BigInt::new(-6789)),big_int::BigInt::new(0));
    }

    #[test]
    fn negative_times_positive_works() {
        assert_eq!(big_int::BigInt::new(-12345).mul(big_int::BigInt::new(6789)),big_int::BigInt::new(12345*-6789));
    }

    #[test]
    fn negative_times_zero_works() {
        assert_eq!(big_int::BigInt::new(-12345).mul(big_int::BigInt::new(0)),big_int::BigInt::new(0));
    }

    #[test]
    fn negative_times_negative_works() {
        assert_eq!(big_int::BigInt::new(-12345).mul(big_int::BigInt::new(-6789)),big_int::BigInt::new(12345*6789));
    }

    #[test]
    fn can_create_simple_big_ints_from_string_positive() {
        assert_eq!(big_int::BigInt::from_string("123".to_string()), big_int::BigInt::new(123))
    }

    #[test]
    fn can_create_big_big_ints_from_string_positive() {
        assert_eq!(big_int::BigInt::from_string("123456789".to_string()), big_int::BigInt::new(123456789))
    }

    #[test]
    fn can_create_really_big_big_ints_from_string_positive() {
        assert_eq!(big_int::BigInt::from_string("123,000,000,000,000".to_string()), big_int::BigInt::new(123).mul(big_int::BigInt::new(1000000)).mul(big_int::BigInt::new(1000000)));
    }

    #[test]
    fn can_create_simple_big_ints_from_string_negative() {
        assert_eq!(big_int::BigInt::from_string("-123".to_string()), big_int::BigInt::new(-123))
    }

    #[test]
    fn can_create_big_big_ints_from_string_negative() {
        assert_eq!(big_int::BigInt::from_string("-123456789".to_string()), big_int::BigInt::new(-123456789))
    }

    #[test]
    fn can_create_really_big_big_ints_from_string_negative() {
        assert_eq!(big_int::BigInt::from_string("-123,000,000,000,000".to_string()), big_int::BigInt::new(-123).mul(big_int::BigInt::new(1000000)).mul(big_int::BigInt::new(1000000)));
    }
}
