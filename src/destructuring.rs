mod tests {
    #[test]
    fn test_bool() {
        let (a, mut b): (bool, bool) = (true, false);
        assert_eq!(a, true);
        println!("a = {:?}, b = {:?}", a, b);

        b = true;
        assert_eq!(a, b);
    }

    struct S {
        e: i32,
    }

    #[test]
    fn test_destruct_and_let() {
        let (a, b, c, d, e);

        (a, b) = (1, 2);
        // this is using the `..` pattern to ignore the rest of the values
        // and the `_` pattern to ignore the value at the final position
        [c, .., d, _] = [1, 2, 3, 4, 5];
        S { e } = S { e: 5 };

        assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
        println!("arr = {:?}", [a, b, c, d, e]);

        let s;
        [_, .., s, _] = [1, 2, 3, 4, 6, 5];

        println!("{}", s)
    }
}