use num::complex::Complex;

fn main() {
    // this num lib is not in the std lib so we need to add it to the Cargo.toml file
    // we can use the num lib to do complex math operations like this example
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}
