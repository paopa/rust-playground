fn main() {
    bool();
    destruct_and_let();
}

fn bool() {
    let (a, mut b): (bool, bool) = (true, false);

    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

struct S {
    e: i32,
}

fn destruct_and_let() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    S { e } = S { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!("arr = {:?}", [a, b, c, d, e]);
}
