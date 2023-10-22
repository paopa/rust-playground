fn main() {
    // binary for 2 is 00000010
    let a:i32 = 2;
    // binary for 3 is 00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // Note that these operators can be added with = for assignment except for ! (because != is used to judge not equal)
    a <<= b;
    println!("(a << b) value is {}", a);
}