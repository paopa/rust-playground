mod external;
mod customer;
mod fun;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// the `pub` keyword in the `use` statement makes the function public
// so that it can be used outside of the module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_short() {
    // we can use `use` keyword to bring the function into scope
    hosting::add_to_waitlist();
}


use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


mod tests {
    use crate::{back_of_house, eat_at_restaurant};

    #[test]
    fn test() {
        eat_at_restaurant();
    }

    #[test]
    fn test_eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }

    #[test]
    fn test_back_of_house() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

mod nest_path {
    // In summary, use std::io; brings the io module into scope, but not its contents.
    // If you want to use a specific item from the io module,
    // you need to bring it into scope explicitly with use std::io::Write;.

    // use std::io;
    // use std::io::Write;
    use std::io::{self, Write};
}