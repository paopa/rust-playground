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
}