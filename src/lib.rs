//! This is a documentation comment as well, to comment containing items
//! (e.g. crates, modules, functions). instead of the items following it.
//! Commonly used inside the crate root file (lib.rs) or modules root file (mod.rs)
//! more documentation comment refer to comment.rs

// this is the root file of the library crate, so it doesn't need a main function
// when the compiler compiles the library crate, it will look for the lib.rs file(usually) and compile it

// declaring a module, the compiler will look for the module's code in these places:
// 1. inline, within curly brackets that replace the semicolon following `mod module_name`
mod garden {
    fn plant() {
        println!("planting a tree");
    }
}
// 2. in the file src/module_name.rs
mod garden_foo;
// 3. in the file src/module_name/mod.rs
mod garden_bar;

mod ownership;
mod unit_test;
// the struct is a keyword in Rust, so we need to use r# to escape it
// https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html
mod r#struct;
mod shadowing;
mod rectangles;
mod enums;
mod option;
mod r#match;
mod functions;
mod destructuring;
mod if_let;
mod comment;