struct User {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let user = User {};
    }
}