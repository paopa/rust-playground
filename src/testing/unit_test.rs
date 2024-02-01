pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

// The tests module is a convention for naming the test module. The tests module is a separate module that Rust knows to treat as a test module.
// to hold your "unit-style" tests. These are small and focused, testing one module in isolation at a time.
// But what about "integration-style" tests instead? For that, we have the `tests` directory.

// why should we add cfg(test) ?
// The cfg attribute is used to conditionally compile code based on a configuration parameter.
// In this case, the test attribute is used to conditionally compile the test code only when the test suite is run.
// This is useful because the test code is not part of the final binary, so it doesn't need to be compiled unless we're running the tests.
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_bad_add_message() {
        assert_eq!("Hello", "world");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        // cargo test -- --ignored to run ignored tests
    }
}