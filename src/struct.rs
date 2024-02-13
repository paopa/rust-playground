#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 0,
    // }
    // if the parameter name is the same as the struct field name, we can use the field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
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

    #[test]
    fn struct_update_syntax() {
        let user1 = build_user(String::from("foo@gmail.com"), String::from("foo"));

        println!("user1: {:?}", user1);

        let email = String::from("foo_boo@gmail.com");
        let user2 = User {
            email, // we can't use email after this line, because the ownership of email has been moved to user2
            ..user1
        };
        // println!("email: {}", email); // this line will cause an error

        println!("email: {}", user1.email); // this line won't be an error, because we don't move the email field
        println!("active: {}", user1.active); // this line won't be an error
        println!("sign_in_count: {}", user1.sign_in_count); // this line won't be an error
        // Both active and sign_in_count are types that implement the Copy trait,
        // so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

        // according to the above, we can use the user1 after the struct update syntax.
        // but we can't use the username field, because it's not a type that implements the Copy trait
        // and the ownership of the username field has been moved to user2
        // println!("username: {}", user1.username); // this line will cause an error

        // and also we can't use the user1 as a whole, because part of the fields have been moved to user2
        // println!("user1: {:?}", user1); // this line will cause an error

        println!("user2: {:?}", user2);
    }

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Demo(i32, String, bool);

    #[test]
    fn tuple_struct() {
        let black = Color(0, 1, 4);
        println!("black: {:?}", black);

        // we can access the value of the tuple struct by using the index
        println!("{}", black.0);
        println!("{}", black.1);
        println!("{}", black.2);

        tuple_struct_as_parameter(black);

        let demo = Demo(1, String::from("hello world"), true);
        println!("demo: {:?}", demo);

        println!("{}", demo.0);
        println!("{}", demo.1);
        println!("{}", demo.2);
    }

    fn tuple_struct_as_parameter(color: Color) {
        println!("print the color {:?} in the function", color);
    }

    // A unit-like struct is a way of using the tuple struct to create a type that is an empty tuple, (),
    // which can be useful in situations in which you need to implement a trait on some type
    // but don’t have any data that you want to store in the type itself.
    #[derive(Debug)]
    struct AlwaysEqual;

    #[test]
    fn unit_like_struct() {
        let always_equal = AlwaysEqual;
        println!("always_equal: {:?}", always_equal);
    }
}