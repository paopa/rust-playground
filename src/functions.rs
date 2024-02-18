// what is the difference between a statement and an expression?
//
// statement: a statement is an instruction that performs some action and does not return a value.
// expression: an expression evaluates to a resulting value.

// Rust is an expression-based language.
// This means that most things are expressions, this includes if/else blocks, loops, and even function definitions.

// a function must return a value, if it does not return a value, it returns the empty tuple ().
fn function1() {
    // this is a statement, because let keyword is used to declare a variable, and it does not return a value.
    let x = println!("Hello, function1");
    // this is an expression, because println! returns a value of type () (unit).
    println!("The value of x is: {:?}", x);

    // () is the empty tuple, it has special name, `unit`, in Rust.
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.

    // this is an expression
    1;
}

fn function2(i: i32) -> i32 {
    // this is an expression
    i + 1
}

fn function3(i: i32) -> i32 {
    // this is an expression
    return i + 1;
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        // this is a statement
        let x = 5;
        // this is a statement
        let y = {
            // the new scope block creates with the curly braces is an expression.
            let x = 3;
            x + 1
        };
        println!("The value of y is: {}", y);
        println!("The value of x is: {}", x);
        println!("The value of function2 is: {}", function2(5));
        println!("The value of function3 is: {}", function3(5));
        function1();
    }
}
