pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("{}という値を得ました", a);
    10
}

pub fn multiply_by_two(a: i32) -> i32 {
    a * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[should_panic]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn multiplying_two_and_two() {
        assert_eq!(4, multiply_by_two(2));
    }

    #[test]
    fn multiplying_three_and_two() {
        assert_eq!(6, multiply_by_two(3));
    }

    #[test]
    fn multiplying_one_hundred() {
        assert_eq!(200, multiply_by_two(100));
    }

    #[test]
    #[ignore]
    fn multiplying_one_thousand() {
        assert_eq!(2000, multiply_by_two(1000));
    }
}
