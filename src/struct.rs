#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let mut user = User {
            // don't need to follow the order of the struct
            sign_in_count: 0,
            active: false,
            username: String::from("user"),
            email: String::from("user@gmail.com"),
        };

        println!("username: {:?}", user);

        // if the struct is mutable, we can change the value of the struct
        user.username = String::from("user2");
    }
}