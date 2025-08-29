pub mod adder;

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_2(a: i32) -> i32 {
    internal_adder(a, 2)
}

// this is private
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // lets us use functions in parent module

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
