// use keyword to bring the module into scope
use rust_playground::garden_bar::vegetables;

// code within a module is private from its parent modules by default.
// to make the code public, use the pub keyword and the related items will be public as well

fn main() {
    vegetables::plant();
}