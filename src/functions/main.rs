// what is the difference between a statement and an expression?
//
// statement: a statement is an instruction that performs some action and does not return a value.
// expression: an expression evaluates to a resulting value.

// Rust is an expression-based language.
// This means that most things are expressions, this includes if/else blocks, loops, and even function definitions.

fn function1() {
    // this is a statement
    println!("Hello, function1");
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

fn main(){
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