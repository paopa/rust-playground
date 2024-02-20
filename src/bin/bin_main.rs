fn main() {
    // the whole project with the Cargo.toml file is a package
    // the package can contain multiple binary crates and at most one library crate

    // the binary crates are the ones that are executed when you run cargo run
    // so, the main.rs and any other file with the main function under the src/bin directory are binary crates
    println!("Hello, world! this is a binary crate");

    // cargo run --bin bin_main to run this binary crate
}