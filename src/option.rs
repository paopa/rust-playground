mod tests {
    #[test]
    fn test_option() {
        let some_number = Some(5);
        let some_char = Some('e');

        assert_eq!(some_number.unwrap(), 5);
        assert_eq!(some_char.unwrap(), 'e');

        // Rust requires us to annotate the overall Option type:
        // the compiler can’t infer the type that the corresponding Some variant
        // will hold by looking only at a None value.
        let absent_number: Option<i32> = None;
        assert_eq!(absent_number.unwrap_or(1), 1);
        println!("{:?}", absent_number);

        // why does the following code compile?
        // because the compiler can infer the type of the Option<i32> value via the unwrap_or method, I believe.
        let absent_number = None;
        assert_eq!(absent_number.unwrap_or(1), 1);
    }
}