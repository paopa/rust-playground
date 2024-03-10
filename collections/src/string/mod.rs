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
}