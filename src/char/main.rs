fn main() {
    /*
     * in Rust, char is 4 bytes in size and represents a Unicode Scalar Value (Unicode Code Point)
     * this means that it can represent a lot more than just ASCII.
     * 
     * What is the difference for char between Rust and Java?
     * In Java, char is 2 bytes in size and represents a UTF-16 code unit. so it can't represent all Unicode Scalar Values.
     * 
     * What is the difference for char between Rust and C++?
     * In C++, char is 1 byte in size and represents a UTF-8 code unit. so it can't represent all Unicode Scalar Values.
     * 
     * What is the difference for char between Rust and Python?
     * In Python, char is 1 byte in size and represents a UTF-8 code unit. so it can't represent all Unicode Scalar Values.
     * 
     * What is the difference for char between Rust and Go?
     * In Go, char is 1 byte in size and represents a UTF-8 code unit. so it can't represent all Unicode Scalar Values.
     * 
     * What is the difference for char between Rust and JavaScript?
     * In JavaScript, char is 2 bytes in size and represents a UTF-16 code unit. so it can't represent all Unicode Scalar Values.
     */
    let _c = 'z';
    let _z = 'ℤ';
    let _g = '国';
    let _heart_eyed_cat = '😻';

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}
