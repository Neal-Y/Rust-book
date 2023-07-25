// use core::fmt::Debug;
// use std::fmt::Display;
// pub trait Summary {
//     fn summarize_author(&self) -> String;
//     fn summarize(&self) -> String {
//         return format!("read more from {}...", self.summarize_author());
//     }
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {
//         return format!("@{}", self.author);
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
//     format!("Breaking news! {}", a.summarize())
// }

// pub fn notify<T, U>(a: T, b: U) -> String
// where
//     T: Summary + Display,
//     U: Clone + Debug,
// {
//     format!("Breaking news! {}", a.summarize())
// }

// pub fn notify(s: &str) -> impl Summary {
//     NewsArticle {
//         headline: String::from("penguins win the "),
//         content: String::from("the Pittsburgh"),
//         author: String::from("sdaf"),
//         location: String::from("asdfasdf"),
//     }
// }
// use std::fmt::Display;

// struct Pait<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pait<T> {
//     fn new(x: T, y: T) -> Self {
//         return Self { x, y };
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("f");
//         } else {
//             println!("t");
//         }
//     }
// }
