// How Rust manages memory?
// memory is managed through a system of ownership with a set of rules that the compiler checks at compile time
// None of the ownership features slow down your program while it’s running (like garbage collector)
// All the analysis is done at compile time (like C++) and doesn’t require a garbage collector (like C++)

// Ownership Rules:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// The ownership rules help Rust prevent
// - dangling pointers
// - double free
// - memory leaks

// There is a link to the Rust Book: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
// I recommend reading the book to understand the ownership in Rust, not only it's a very important concept in Rust,
// but also it's a very interesting concept in programming languages, and it's very well explained in the book

#[cfg(test)]
mod tests {
    #[test]
    fn variable_scope() {
        { // s is not valid here, it’s not yet declared
            let s = String::from("hello"); // s is valid from this point forward
            // do stuff with s
        } // this scope is now over, and s is no longer valid
    }

    #[test]
    fn variable_and_data_interacting_with_move() {
        // I think we could imagine that was like a shallow copy
        // because the variable s1 is moved to s2, so s1 is no longer valid
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world!", s1); // This line will not compile
        //  ^^ value borrowed here after move

        println!("{}, world!", s2);
    }

    #[test]
    fn variable_and_data_interacting_with_clone() {
        // I think we could imagine that was like a deep copy
        // because the variable s1 is cloned to s2, so s1 is still valid
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // in this case, both s1 and s2 are valid, because s1 is cloned to s2
        // so, s1 didn't move the ownership to s2, but clone a new one is expensive in terms of memory
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    #[test]
    fn stack_only_data_copy() {
        // why this works?
        // the reason is that types such as integers that have a known size at compile time
        // are stored entirely on the stack, so copies of the actual values are quick to make
        let x = 5;
        let y = x;
        // in this case, both x and y are valid, because x is copied to y
        // so, x didn't move the ownership to y
        println!("x = {}, y = {}", x, y);

        // what is the Copy trait?
        // If a type has the Copy trait, an older variable is still usable after assignment
        // because the data is copied, not moved (like in the case of String)
        // Rust won’t let us annotate a type with the Copy trait if the type,
        // or any of its parts, has implemented the Drop trait

        // Here are some of the types that implement the Copy trait:
        // - All the integer types, such as u32.
        // - The Boolean type, bool, with values true and false.
        // - All the floating point types, such as f64.
        // - The character type, char.
        // - Tuples, if they only contain types that also implement Copy.
        //   For example, (i32, i32) implements Copy, but (i32, String) does not.
    }

    #[test]
    fn ownership_and_functions() {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s);             // s's value moves into the function...
        // ... and so is no longer valid here
        // println!("{}, world!", s); // This line will not compile
        //  ^^ value borrowed here after move

        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function,
        // but i32 is Copy, so it’s okay to still
        // use x afterward
        println!("x = {}", x);
    }   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    }   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    #[test]
    fn return_values_and_scope() {
        let s1 = gives_ownership();        // gives_ownership moves its return value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
        // which also moves its return value into s3.
    } // Here, s3 goes out of scope and is dropped.
    // s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it

        let some_string = String::from("yours"); // some_string comes into scope

        some_string    // some_string is returned and moves out to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

        a_string  // a_string is returned and moves out to the calling function
    }
}