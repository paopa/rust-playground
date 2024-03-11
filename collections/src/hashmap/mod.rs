#[cfg(test)]
mod tests {
    // By default, HashMap uses a hashing function called SipHash that is designed to prevent
    // certain types of denial-of-service (DoS) attacks. This is not the fastest hashing algorithm,
    // but the trade-off is worth it for the security it provides.
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn test_entry() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));

        let entry = map.entry("d");
        assert_eq!(entry.or_insert(4), &4);
        map.entry("d").or_insert(5); // no effect because "d" already exists

        println!("{:?}", map);
    }
}