mod test {
    #[test]
    fn test_if_let() {
        let some_u8_value = Some(10u8);

        // if let is a syntax sugar for a match that runs code when the value matches one pattern
        // and then ignores all other values. the following two snippets are equivalent:

        match some_u8_value {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }

        if let Some(max) = some_u8_value {
            println!("The maximum is configured to be {}", max);
        }
    }
}