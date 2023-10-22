fn main() {
    /*
     * Range is a type that defines a start and an end.
     * It is defined by two dots .. and called range operator.
     */

    let _x = 1..10;

    // range from 1 to 10 (exclusive)
    for i in 1..10 {
        println!("{}", i);
    }

    print!("====================\n");
    // range from 1 to 10 (inclusive)
    for i in 1..=10 {
        println!("{}", i);
    }

    print!("====================\n");
    // range from a to e (inclusive)
    for c in 'a'..='e' {
        println!("{}", c);
    }
}
