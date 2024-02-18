#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // we can destructure the state from the quarter
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match is exhaustive, so we need to handle all the cases to avoid a compile error
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn catch_all_bind_var(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        other => println!("not match: {}", other), // other is a catch-all, and will put the value in the variable
    }
}

fn catch_all_dont_bind_var(num: u8) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("not match"), // _ is a catch-all, but it doesn't put the value in a variable.
        // so it's used when we don't care about the value. and if we use _ in the last arm,
        // we will occur a compile error.
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_value_in_cents() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)), 25);
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(Some(5)), Some(6));
        assert_eq!(plus_one(None), None);
    }

    #[test]
    fn test_catch_all_bind_var() {
        catch_all_bind_var(1);
        catch_all_bind_var(3);
        catch_all_bind_var(5);
        catch_all_bind_var(7);
        catch_all_bind_var(9);
    }

    #[test]
    fn test_catch_all_dont_bind_var() {
        catch_all_dont_bind_var(1);
        catch_all_dont_bind_var(3);
        catch_all_dont_bind_var(5);
        catch_all_dont_bind_var(7);
        catch_all_dont_bind_var(9);
    }
}