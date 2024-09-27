// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_eq_pass() {
        assert_eq!(3, 3);
    }

    #[test]
    fn test_assert_eq_fail() {
        assert_eq!(3, 4); 
    }
}
