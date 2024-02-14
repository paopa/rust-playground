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

impl Rectangle {
    // the &self is actually shorthand for &self: &Self,
    // where Self is the type of the struct the method is being defined on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // if we wanted to change the instance that we’ve called the method on as part of what the method does,
    // we’d use `&mut self` as the first parameter.
    fn width(&self) -> bool {
        self.width > 0
    }

    // having a method that takes ownership of the instance by using just `self` as the first parameter is rare;
    // this technique is usually used when the method transforms `self` into something else,
    // and you want to prevent the caller from using the original instance after the transformation.
    fn consume(self) {
        // this method takes ownership of the instance and does something that consumes the instance.
        // after calling this method, the instance is no longer valid.
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // what is the difference between associated functions and methods?
    // Associated functions are functions that are associated with the struct,and don’t take a self parameter.
    // They’re still functions, not methods, because they don’t have an instance of the struct to work with.
    // so, associated functions are often used for constructors that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    // is this an associated function?
    // I think it is, because it doesn't take a self parameter. and be defined in the impl block of the struct.
    fn test(size: u32) -> u32 {
        size
    }
}

impl Rectangle {
    fn hello(&self) -> String {
        "hello Rectangle".to_string()
    }
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

    #[test]
    fn test_area_method() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.area(), 1500);

        if rect1.width() {
            println!("rect1 has a width");
        }

        println!("rect1 is {:#?}", rect1);

        rect1.consume(); // it moves the ownership of rect1 to the consume method
        // println!("rect1 is {:#?}", rect1); // this will occur a compile error
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        assert!(rect1.can_hold(&rect2));
        println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
        assert!(!rect1.can_hold(&rect3));
        println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
    }

    #[test]
    fn test_associated_functions() {
        let square = Rectangle::square(3);
        assert_eq!(square.width, 3);
        assert_eq!(square.height, 3);

        let size = Rectangle::test(3);
        assert_eq!(size, 3);
    }

    #[test]
    fn test_multiple_impl_blocks() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.hello(), "hello Rectangle");
    }
}