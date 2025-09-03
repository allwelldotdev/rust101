// Using the pattern syntax.

fn main() {
    // // Matching Literals
    // // This syntax is useful when you want your code to take an action if it gets a particular concrete value.
    // let x = 1;
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // --------------

    // // Matching Named Variables
    // let x = Some(5);
    // let y = 10;
    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Matched, y = {y}"),
    //     _ => println!("Default case, x = {x:?}"),
    // }
    // println!("at the end: x = {x:?}, y = {y}");

    // --------------

    // // Matching Multiple Values
    // let x = 1;
    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // ----------

    // // Matching Ranges of Values with `..=`
    // let x = 5;
    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }

    // let x = 'c';
    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("early ASCII letter"),
    //     _ => println!("something else"),
    // }

    // -------------

    // Destructuring to Break Apart Values

    // // Destructuring Structs
    // let p = Point { x: 0, y: 7 };
    // match p {
    //     Point { x, y: 0 } => println!("On the x axis at {x}"),
    //     Point { x: 0, y } => println!("On the y axis at {y}"),
    //     Point { x, y } => println!("On neither axis: ({x}, {y})"),
    // }

    // ----------

    // // Destructuring Enums
    // let msg = Message::ChangeColor(0, 160, 255);
    // match msg {
    //     Message::Quit => println!("The quit variant has no data to destructure."),
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {x} and in the y direction {y}");
    //     }
    //     Message::Write(text) => println!("Text message: {text}"),
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change color to red {r}, green {g}, and blue {b}");
    //     }
    // }

    // ------------

    // // Destructuring Nested Structs and Enums
    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // match msg {
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => {
    //         println!("Change color to red {r}, green {g}, and blue {b}");
    //     }
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => {
    //         println!("Change color to hue {h}, saturation {s}, and value {v}");
    //     }
    //     _ => (),
    // }

    // ------------

    // // Destructuring Structs and Tuples
    // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // assert_eq!(3, feet);
    // assert_eq!(10, inches);
    // assert_eq!(3, x);
    // assert_eq!(-10, y);

    // -------------

    // Ignoring Values in a Pattern

    // // An Entire Value with `_`
    // foo(3, 4);

    // ------------

    // // Renaming Parts of a Value with `..`
    // let origin = Point { x: 0, y: 0, z: 0 };
    // match origin {
    //     Point { x, .. } => println!("x is {x}"),
    // }

    // let (first, .., last) = (2, 4, 8, 16, 32);
    // println!("Some numbers: {first}, {last}");

    // ----------

    // // Extra Conditionals with Match Guards
    // let num = Some(4);
    // match num {
    //     Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    //     Some(x) => println!("The number {x} is odd"),
    //     None => (),
    // }

    // let x = 4;
    // let y = false;
    // match x {
    //     4 | 5 | 6 if y => println!("yes"),
    //     _ => println!("no"),
    // }

    // ---------------

    // @ Bindings
    // Using `@` lets us test a value and save it in a variable within one pattern
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

#[allow(unused)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// #[allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

#[allow(unused)]
enum Message {
    Hello { id: i32 },
}

#[allow(unused)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(unused)]
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
