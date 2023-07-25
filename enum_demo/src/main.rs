// enum IpAddKind {
//     V4,
//     V6,
// }

// fn main() {
//     let four = IpAddKind::V4;
//     let six = IpAddKind::V6;

//     route(four);
//     route(six);
//     route(IpAddKind::V6);
//     //? 都可以
//     println!("Hello, world!");
// }

// fn route(ip_kind: IpAddKind) {}

// enum IpAddKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddKind::V4(127, 0, 0, 1);
//     let loopback = IpAddKind::V6(String::from("::1"));
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {}
// }
// fn main() {
//     let q = Message::Quit;
//     let m = Message::Move { x: 12, y: 12 };
//     let w = Message::Write(String::from("::12"));
//     let c = Message::ChangeColor(0, 0, 0);

//     m.call();
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];
//     let result = largest(&numbers);
//     println!("The largest number is {}", result);

//     let chars = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&chars);
//     println!("The largest char is {}", result);
// }

// trait Shape {
//     fn area(&self) -> f64;
// }

// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Shape for Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }

// struct Circle {
//     radius: f64,
// }

// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }

// fn main() {
//     let shapes: Vec<Box<dyn Shape>> = vec![
//         Box::new(Rectangle {
//             width: 10.0,
//             height: 20.0,
//         }),
//         Box::new(Circle { radius: 5.0 }),
//     ];

//     for shape in shapes {
//         println!("Area: {}", shape.area());
//     }
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return None;
    } else {
        return Some(denominator / numerator);
    }
}

fn main() {
    let result = divide(5.0, 2.0);
    //* None

    match result {
        Some(ref result) => println!("值{}", result), //? ref 指針
        None => println!("木有"),
    }

    //* ' _ ' underscore

    match result {
        Some(ref result) => println!("值{}", result),
        _ => (),
    }

    //* if let 損失一些窮舉的可能沒有match的全面，但簡潔。
    if let Some(result) = result {
        println!("值{}", result);
    } else {
        println!("啥也不是");
    }
}
