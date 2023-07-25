// pub struct AveragedCollection {
//     list: Vec<i32>,
//     average: f64,
// }

// impl AveragedCollection {
//     pub fn add(&mut self, value: i32) {
//         self.list.push(value);
//         self.update_average();
//     }

//     pub fn remove(&mut self) -> Option<i32> {
//         let result = self.list.pop();
//         match result {
//             Some(value) => {
//                 self.update_average();
//                 Some(value)
//             }
//             None => None,
//         }
//     }

//     pub fn average(&self) -> f64 {
//         self.average
//     }

//     fn update_average(&mut self) {
//         let total: i32 = self.list.iter().sum();
//         self.average = total as f64 / self.list.len() as f64;
//     }
// }

// trait Shape {
//     fn area(&self) -> f64;
// }

// struct Circle {
//     radius: f64,
// }

// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }

// impl Shape for Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let shapes: Vec<Box<dyn Shape>> = vec![
//         Box::new(Circle { radius: 1.0 }),
//         Box::new(Rectangle {
//             width: 2.0,
//             height: 3.0,
//         }),
//         Box::new(Circle { radius: 2.5 }),
//     ];

//     for shape in shapes.iter() {
//         println!("Area of shape is {}", shape.area());
//     }
// }

// trait Animal {
//     fn make_sound(&self);
// }

// struct Cat;
// impl Animal for Cat {
//     fn make_sound(&self) {
//         println!("Meow");
//     }
// }

// struct Dog;
// impl Animal for Dog {
//     fn make_sound(&self) {
//         println!("Woof");
//     }
// }

// fn main() {
//     let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
//     for animal in animals.iter() {
//         animal.make_sound();
//     }
// }

// use std::vec;

// use OOOOP::Draw;
// use OOOOP::{Button, Screen};

// pub struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     // fn draw(&self) {
//     //     //?
//     // }
// }

// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("yes"),
//                     String::from("maybe"),
//                     String::from("NO"),
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }

fn main(){
    println!("hello world");
}