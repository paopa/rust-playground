#[cfg(test)]
mod tests {
    #[test]
    fn test_string() {
        let mut s = String::new();
        assert_eq!(s, "");

        s.push('H');
        assert_eq!(s, "H");

        let data = "initial contents";
        let s = data.to_string();
        assert_eq!(s, "initial contents");

        let s = "hello".to_string();
        assert_eq!(s, "hello");

        let s = String::from("hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_update_string() {
        let mut s = String::from("foo");
        s.push_str("bar");
        assert_eq!(s, "foobar");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        assert_eq!(s1, "foobar");
        assert_eq!(s2, "bar");

        let mut s = String::from("lo");
        s.push('l');
        assert_eq!(s, "lol");
    }

    #[test]
    fn test_concatenate_string() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        // s1 has been moved here and can no longer be used
        // the `+` operator uses the `add` method, which takes ownership of `s1` and does not return it
        assert_eq!(s3, "Hello, world!");
        assert_eq!(s2, "world!");

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        assert_eq!(s, "tic-tac-toe");

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        assert_eq!(s, "tic-tac-toe");
    }

    #[test]
    fn test_index_string() {
        let hello = "Здравствуйте";
        let answer = &hello.as_bytes()[0];
        // if you run the code below:
        //
        // let h = hello[0];
        //
        // you will get an error because Rust strings don't support indexing.
        // But why? The reason is that Rust string is a wrapper over a Vec<u8>.
        // so, when you try to get the ownership of an element in the string, it will occur an error.
        assert_eq!(*answer, 208);
    }

    #[test]
    fn test_bytes_scalar_value_grapheme() {
        let s = "नमस्ते";

        let bytes = s.as_bytes(); //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
        println!("{:?}", bytes);
        assert_eq!(bytes.len(), 18);

        let scalar_values = s.chars(); // ['न', 'म', 'स', '्', 'त', 'े']
        println!("{:?}", scalar_values);
        assert_eq!(scalar_values.count(), 6);

        // There are six char values here, but the fourth and sixth are not letters.
        // they're diacritics that don't make sense on their own. Finally, if we look at them as grapheme clusters,
        // we'd get what a person would call the four letters that make up the Hindi word: ["न", "म", "स्", "ते"]
        //
        // Rust provides different ways of interpreting the raw string data that computers store
        // so that each program can choose the interpretation it needs, no matter what human language the data is in.

    }
}