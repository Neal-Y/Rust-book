// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();

//     if let Some(color) = favorite_color {
//         println!("Using your favorite color, {color}, as the background");
//     } else if is_tuesday {
//         println!("Tuesday is green day!");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("Using purple as the background color");
//         } else {
//             println!("Using orange as the background color");
//         }
//     } else {
//         println!("Using blue as the background color");
//     }
// }

// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// struct Example {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let x = Example { x: 3, y: 4 };
//     let Example { x, y } = x;
//     assert_eq!(x, 3);
//     assert_eq!(y, 4);
// }

// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change color to red {r}, green {g}, and blue {b}");
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change color to hue {h}, saturation {s}, value {v}")
//         }
//         _ => (),
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
// }

// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {}", y);
// }

// fn main() {
//     foo(3, 4);
// }
// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }

//     println!("setting is {:?}", setting_value);
// }

// fn main() {
//     struct Point {
//         x: i32,
//         y: i32,
//         z: i32,
//     }

//     let origin = Point { x: 0, y: 0, z: 0 };

//     match origin {
//         Point { x, .. } => println!("x is {}", x),
//     }
// }

// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x % 2 == 0 => println!("The number {} is even", x),
//         Some(x) => println!("The number {} is odd", x),
//         None => (),
//     }
// }

// fn main() {
//     let x = 4;
//     let y = false;

//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n = {n}"),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {y}", x);
// }

#[derive(Debug)]
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
            // println!("Found an id in another range{}", id_variable);
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    println!("Found an id in another range{:?}", msg);
    let x = Some(5);
    match x {
        Some(_n @ 5..=10) => println!("fadsf"),
        _ => println!("dasf"),
    }
}
