fn println_rectangle_variables() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_variables(width, height)
    );
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}

fn println_rectangle_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn println_rectangle_struct() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle { width: dbg!(10), height: 40 };
    dbg!(&rect2);
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

mod tests {
    use super::*;

    #[test]
    fn test_area_variables() {
        assert_eq!(area_variables(30, 50), 1500);
        println_rectangle_variables();
    }

    #[test]
    fn test_area_tuple() {
        assert_eq!(area_tuple((30, 50)), 1500);
        println_rectangle_tuple();
    }

    #[test]
    fn test_area_struct() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(area_struct(&rect1), 1500);
        println_rectangle_struct();
    }
}