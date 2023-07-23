// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(5==5);
    }

    #[test]
    #[should_panic]
    fn you_can_assert1() {
        assert!(5 != 5);
    }

    #[test]
    fn you_can_assert2() {
        assert_eq!(5, 5);
    }

    #[test]
    fn you_can_assert3() {
        assert_ne!(5, 7);
    }

    #[test]
    fn you_can_assert4() -> Result<(), std::num::ParseIntError> {
        let int: i32 = "32".parse()?;
        Ok(())
    }
}
