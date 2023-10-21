fn main() {
    let a: u8 = 255;

    let b: u8 = a.wrapping_add(20);
    println!("b = {}", b);

    let b = a.overflowing_add(20);
    println!("b = {:?}", b);

    let b: u8 = a.saturating_add(20);
    println!("b = {}", b);

    let b: u8 = a.checked_add(20).expect("Overflow!");
    println!("b = {}", b);
}
