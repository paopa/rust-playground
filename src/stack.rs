fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{:?}", stack);
    let e1 = stack.pop();
    let e2 = stack.pop();

    println!("{:?}, {:?}", e1, e2);
    println!("{:?}", stack);
}
