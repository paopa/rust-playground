fn main() {
    println!("Hello, world!");
    println!("testing");

    // let (x, y);

    // (x, ..) = 5;
    // (.., y) = 5;

    // println!("x = {}, y = {}", x, y);
    test();

    let mut arr = [1, 5, 3, 2, 4];
    bubble_sort(&mut arr);
}

fn test() {
    println!("this it");
}

// create a function to do bubble sort on a integer array
// and don't return anything but print the sorted array
fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        n = n - 1;
    }

    println!("sorted array: {:?}", arr);
}
