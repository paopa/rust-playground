// In Rust, the asterisk (*) syntax is used for dereferencing,
// which means accessing the value that a reference points to.
// Let’s go through the example code step-by-step to understand this concept:
fn main() {
    let x = 5; // `x` is an integer with value 5
    let y = &x; // `y` is a reference to `x`, the type of y is `&i32`
    let z = *y; // `z` is the value `y` is pointing to, which is `x`
    println!("x: {}, y: {:p}, z: {}", x, y, z);
    println!("x: {}, y: {}, z: {}", x, y, z);
}
