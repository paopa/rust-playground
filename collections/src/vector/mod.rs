#[cfg(test)]
mod tests {
    #[test]
    fn test_vector() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
        println!("{:?}", v);
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        // using get method to access elements in vector to avoid panic
        // when index is out of range. and it is more common to use get method
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    #[test]
    fn test_vector_macro() {
        let v = vec![1, 2, 3, 4, 5];
        println!("{:?}", v);
    }

    #[test]
    fn test_vector_borrow_check() {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
        // this error is due to the way vector works, when we push a new element into the vector, it might
        // need to allocate new memory and copy the old elements to the new space, if that happens,
        // the reference to the first element would be pointing to deallocated memory.
        println!("The first element is: {}", first);
    }

    #[test]
    fn test_vector_iter() {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    #[test]
    fn test_vector_iter_mut() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // dereference i to get the value it refers to, and then add 50 to that value
            *i += 50;
        }
        println!("{:?}", v);
    }
}