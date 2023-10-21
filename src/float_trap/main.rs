fn main() {
    /*
    the root cause of the problem is that 0.1 cannot be represented exactly in binary floating-point.
    The value we actually get is the closest representable value. This is true of all floating-point
    numbers, but it’s particularly relevant to 0.1 because of the way decimal fractions are represented
    in binary floating-point. This is the same reason that 1.0/10.0 does not produce a value that is
    exactly 0.1. The value is very close to 0.1, but it’s not exactly 0.1.
     */
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}
