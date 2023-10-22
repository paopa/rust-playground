fn main() {
    /*
     * NaN (Not a Number) is a special floating-point value that is used to represent a value that is not a number.
     * It is the result of an invalid operation, such as dividing zero by zero. NaN is often used to represent the
     * result of operations that do not yield a meaningful result when applied to particular operands.
     */
    let x = (-42.0_f32).sqrt();

    if x.is_nan() {
        println!("{} is not a number!", x);
    }
}
