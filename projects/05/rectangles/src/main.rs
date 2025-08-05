// An example program using structs
// Take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactoring with Tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }

// Refactoring with Structs

#[derive(Debug)] // adding Debug trait to enable format in Debug mode
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // println!("rect1 is {:#?}", rect1); // format in Debug mode

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
