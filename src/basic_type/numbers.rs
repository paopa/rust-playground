fn main() {
    println!("Hello, numbers file");
    remove_sth_make_it_work();
    fill_the_blank();
    modify_assert_eq();
}

// Remove something to make it work
fn remove_sth_make_it_work() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}

// Fill the blank
fn fill_the_blank() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Modify `assert_eq!` to make it work
fn modify_assert_eq() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
