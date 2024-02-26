mod eating;

// in this case, we have to use `super` keyword to go up one module
// and then to direct to the `front_of_house` module. we can imagine
// this as a file system.
use super::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
