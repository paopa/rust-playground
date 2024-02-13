/*
* Shadowing is different from marking a variable as mut,
* because we’ll get a compile-time error if we accidentally try to reassign to this variable
* without using the let keyword.
* By using let, we can perform a few transformations on a value
* but have the variable be immutable after those transformations have been completed.
*/
#[cfg(test)]
mod tests {
    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }

        println!("The value of x in the outer scope is: {}", x);

        let spaces = "   ";
        let spaces = spaces.len();

        println!("The value of spaces is: {}", spaces)
    }

    #[test]
    fn test_reassign_to_diff_type() {
        let mut spaces = "   ";
        // we can't reassign spaces to a different type
        // spaces = spaces.len();
    }
}
