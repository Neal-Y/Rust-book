// fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }

// fn main() {
//     let number_list = [34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("this is largest {}", result);

//     let number_list = vec!['a', 'b', 's', 'w', 'p'];
//     let result = largest(&number_list);
//     println!("this is largest {}", result);
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer = Point { x: 0, y: 0.0 };
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         return &self.x;
//     }
// }
// impl Point<i32> {
//     fn x1(&self) -> &i32 {
//         return &self.x;
//     }
// }
// fn main() {
//     let integer = Point { x: 0.0, y: 0.0 };
// }
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         return Point {
//             x: self.x,
//             y: other.y,
//         };
//     }
// }
// fn main() {
//     let p1 = Point { x: 5, y: 4 };
//     let p2 = Point { x: "hello", y: 'c' };
//     let p3 = p1.mixup(p2);

//     println!("{}   {}", p3.x, p3.y);
// }

//? trait
// use template::{NewsArticle, Summary};

// fn main() {
//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     println!("New article available! {}", article.summarize());
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you know"),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 new tweet: {}", tweet.summarize());
// }
//? lifetime
// fn main() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//         println!("r:{}", r);
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }

//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// fn main() {
//     let novel = String::from("call me.sdfa");

//     let first_sentence = novel.split('.').next().expect("could not found a '.'");

//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result =
//         longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
//     println!("The longest string is {}", result);
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// pub trait Summary {
//     fn summarize(&self) {}
// }

// struct Tweet {
//     x: String,
//     y: String,
// }
// impl Summary for Tweet {}
// fn return_summary() -> impl Summary {
//     Tweet {
//         x: String::from("sdf"),
//         y: String::from("fsdaf"),
//     }
// }
