fn main() {
    // compiler will infer the type of `twenty` as i32
    let twenty = 20;
    // type annotation can be used to explicitly specify the type of `twenty_one` as i32
    let twenty_one: i32 = 21;
    // suffix annotation can also be used to explicitly specify the type of `twenty_two` as i32
    // and no matter having how much underscores, it will be ignored underscores
    let twenty_two = 22i32; // e.g. 22_i32, 22__i32, or 22______i32 are all the same

    // only i32 values can be added together
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // using underscores to improve readability of numeric literals (since Rust 1.1)
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // define a float array, and 42.0 will be inferred as f32 type because of other elements are f32 type
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // print the element and control the precision of the float number to 2
    println!("{:2}", forty_twos[0]);
}
