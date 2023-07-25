// fn main() {
//     let mut x = 5;

//     let a = &x as *const i32;
//     let b = &mut x as *mut i32;
//     let c = &b as *const *mut i32;
//     // let d = &c;
//     // println!("{}", *a);

//     // unsafe {
//     println!("{:?}", **d);
//     // }
//     drop(a);
//     drop(b);
//     drop(c);
// }
// fn main() {
//     let mut x = 5;

//     let a = &x as *const i32;
//     let b = &mut x as *mut i32;
//     let c = &b as *const *mut i32;

//     let mut_ptr_ptr = unsafe { *c };
//     let mut_ptr = unsafe { *mut_ptr_ptr };

//     unsafe {
//         println!("{:?}", **c);
//     }
//     drop(a);
//     drop(b);
//     drop(c);
//     let mut x = vec![2, 3, 4];
//     let y = &mut x[..];
//     let (u, i) = y.split_at_mut(1);
//     println!("{:?}{:?}", u, i);
// }

// fn main() {
//     let mut num = 5;

//     let ptr1 = &mut num as *mut i32;
//     let ptr2 = &ptr1 as *const *mut i32;

//     unsafe {
//         **ptr2 += 5;
//         println!("num = {}", num);
//     }
//     drop(ptr1);
//     drop(ptr2);
// }

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..]) //? 事實上是可以的，但基於編譯器的嚴謹導致誤殺
// }

// use std::slice;
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let arr_len = slice.len();
//     let ptr = slice.as_mut_ptr();

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), arr_len - mid),
//         )
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let r = &mut v[..];
//     let (l, r) = split_at_mut(r, 2);
//     assert_eq!(l, &mut [1, 2]);
//     assert_eq!(r, &mut [3, 4, 5]);
// }

// extern "C" {
//     fn add(input: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("fasdfasdf {}", add(2));
//     }
// }

// #[no_mangle]
// pub extern "C" fn call_from() {
//     println!("just called a rust function from C!");
// }
// static HELLO_WORLD: &str = "hello world";

// trait Shape {
//     type T;

//     fn area(&self) -> f64;
// }

// struct Circle {
//     radius: f64,
// }

// impl Shape for Circle {
//     type T = f64;

//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
// }
// fn main() {
//     let c = Circle { radius: 2.0 };
//     println!("The area of the circle is {}", c.area());
// }

// use std::ops::Add;
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Self) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }
// impl Add<(i32, i32)> for Point {
//     type Output = Point;

//     fn add(self, other: (i32, i32)) -> Point {
//         Point {
//             x: self.x + other.0,
//             y: self.y + other.1,
//         }
//     }
// }

// fn main() {
//     let x = Point { x: 3, y: 4 };
//     let y = Point { x: 2, y: 1 };
//     println!("{:?}", x + (2, 4));
// }
// use std::ops::Add;

// fn sum<T: Add<Output = T> + Default + Copy>(items: &[T]) -> T {
//     let mut total = T::default();
//     for item in items {
//         total = total + *item;
//     }
//     total
// }

// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let result = sum(&nums);
//     println!("Sum of nums is: {}", result);

//     let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
//     let result = sum(&floats);
//     println!("Sum of floats is: {}", result);

//     let string = "helloworld";
//     let result = sum(&string);
//     println!("Sum of string is: {}", result);
// }

// trait Summable {
//     type Output;
//     fn add(self, other: Self) -> Self::Output;
// }

// impl Summable for i32 {
//     type Output = i32;
//     fn add(self, other: Self) -> Self::Output {
//         self + other
//     }
// }

// impl Summable for f64 {
//     type Output = f64;
//     fn add(self, other: Self) -> Self::Output {
//         self + other
//     }
// }

// impl Summable for char {
//     type Output = u32;
//     fn add(self, other: Self) -> Self::Output {
//         (self as u32) + (other as u32)
//     }
// }

// fn sum<T, U>(items: &[T]) -> U
// where
//     T: Summable<Output = U> + Copy,
//     U: Default + Copy,
// {
//     let mut total = U::default();
//     for item in items {
//         total = item.add(total);
//     }
//     total
// }

// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let result = sum(&nums);
//     println!("Sum of nums is: {}", result);

//     let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
//     let result = sum(&floats);
//     println!("Sum of floats is: {}", result);

//     let string = "helloworld";
//     let chars: Vec<char> = string.chars().collect();
//     let result = sum(&chars);
//     println!("Sum of string is: {}", result);
// }

// use std::ops::Add;

// trait Summable {
//     fn sum(&self) -> i32;
// }

// impl Summable for str {
//     fn sum(&self) -> i32 {
//         self.chars().map(|c| c as i32).sum()
//     }
// }

// fn sum<T, I>(items: I) -> T
// where
//     T: Add<Output = T> + Default + Copy,
//     I: AsRef<[T]>,
// {
//     let items_ref = items.as_ref();
//     let mut total = T::default(); //? 就是那個單位的基礎值
//     for item in items_ref {
//         total = total + *item;
//     }
//     total
// }

// fn main() {
//     let nums = vec![1, 2, 3, 4, 5];
//     let result = sum(nums);
//     println!("Sum of nums is: {}", result);

//     let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
//     let result = sum(floats);
//     println!("Sum of floats is: {}", result);

//     let string = "helloworld";
//     let result = string.sum();
//     println!("Sum of string is: {}", result);
// }
// trait Processor<T> {
//     type Output;
//     fn process(&self, input: T) -> Self::Output;
// }

// struct IntToStringProcessor;

// impl Processor<i32> for IntToStringProcessor {
//     type Output = String;

//     fn process(&self, input: i32) -> String {
//         input.to_string()
//     }
// }

// struct StringToIntProcessor;

// impl Processor<String> for StringToIntProcessor {
//     type Output = i32;

//     fn process(&self, input: String) -> i32 {
//         input.parse().unwrap_or(0)
//     }
// }

// fn main() {
//     let int_processor = IntToStringProcessor;
//     let result = int_processor.process(42);
//     println!("Processed int: {}", result);

//     let string_processor = StringToIntProcessor;
//     let result = string_processor.process("42".to_string());
//     println!("Processed string: {}", result);
// }

// trait Storage {
//     type Item;
//     fn get(&self, index: usize) -> Self::Item;
// }

// struct VecStorage<T> {
//     data: Vec<T>,
// }

// impl<T> Storage for VecStorage<T> {
//     type Item = T;

//     fn get(&self, index: usize) -> Self::Item {
//         self.data[index].clone()
//     }
// }

// trait Transform {
//     type Input;
//     type Output;
//     fn transform(&self, input: Self::Input) -> Self::Output;
// }

// struct StringToInt;

// impl Transform for StringToInt {
//     type Input = String;
//     type Output = i32;

//     fn transform(&self, input: String) -> i32 {
//         input.parse().unwrap_or(0)
//     }
// }

// struct IntToString;

// impl Transform for IntToString {
//     type Input = i32;
//     type Output = String;

//     fn transform(&self, input: i32) -> String {
//         input.to_string()
//     }
// }

// use std::ops::Add;

// struct Millimeters(u32);
// struct Meter(u32);

// impl Add<Meter> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, rhs: Meter) -> Self::Output {
//         Millimeters(self.0 + (rhs.0 * 1000))
//     }
// }

// trait Vehicle {
//     fn honk(&self);
// }

// trait Car: Vehicle {
//     fn start_engine(&self);
// }

// struct Sedan;

// impl Vehicle for Sedan {
//     fn honk(&self) {
//         println!("Honk! Honk!");
//     }
// }

// impl Car for Sedan {
//     fn start_engine(&self) {
//         println!("Engine started.");
//     }
// }

// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }
// #![feature(never_type)]
// fn process_result(result: Result<u32, !>) -> u32 {
//     match result {
//         Ok(value) => value,
//         Err(never) => match never {}, // `!` 類型表示這個分支永遠不會發生
//     }
// }

// fn handle_error() -> ! {
//     panic!("This error should never happen!");
// }
// fn main_loop() -> ! {
//     loop {
//         // ... 執行操作系統的主要任務 ...
//     }
// }
// fn process_result(result: Result<u32, !>) -> u32 {
//     match result {
//         Ok(value) => value,
//         Err(never) => match never {}, // `!` 類型表示這個分支永遠不會發生
//     }
// }

// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg = Message::Hello { id: 121 };

//     match msg {
//         Message::Hello {
//             id: id_variable @ 3..=7,
//         } => {
//             println!("Found an id in range: {}", id_variable)
//         }
//         Message::Hello { id: 10..=12 } => {
//             println!("Found an id in another range")
//         }
//         Message::Hello { id } => {
//             println!("Found some other id: {}", id)
//         }
//     }
// }
// fn main() {
//     let number = Some(41);

//     match number {
//         Some(value @ 42) => println!(
//             "The answer to life, the universe, and everything is {}.",
//             value
//         ),
//         Some(value) => println!("The value is {}.", value),
//         None => println!("No value."),
//     }
// }

// enum Event {
//     Click { x: i32, y: i32 },
//     KeyPress(char),
//     None,
// }

// fn main() {
//     let event = Event::Click { x: 10, y: 20 };

//     match event {
//         Event::Click {
//             y: wwe @ 20..=30, ..
//         } => {
//             println!("Click event at x: {}, y in the range 20..=30: ", wwe)
//         }
//         Event::Click { x, y } => println!("Click event at x: {}, y: {}", x, y),
//         Event::KeyPress(key @ 'a'..='z') => println!("Lowercase letter key press: {}", key),
//         Event::KeyPress(key) => println!("Key press: {}", key),
//         Event::None => println!("No event."),
//     }
//     let x = String::from("x");
// }

// use std::collections::HashMap;
// use std::hash::Hash;

// struct Cache<K>(HashMap<K, Box<[u8]>>)
// where
//     K: Eq + Hash;

// impl<K> Cache<K>
// where
//     K: Eq + Hash,
// {
//     fn new() -> Self {
//         Cache(HashMap::new())
//     }

//     fn insert(&mut self, key: K, value: Box<[u8]>) {
//         self.0.insert(key, value);
//     }

//     fn get(&self, key: &K) -> Option<&[u8]> {
//         self.0.get(key).map(AsRef::as_ref)
//     }
// }

// fn main() {
//     let mut cache: Cache<&str> = Cache::new();

//     cache.insert("foo", Vec::from(&"hello"[..]).into_boxed_slice());
//     cache.insert("bar", "world".as_bytes().to_vec().into_boxed_slice());

//     println!("foo: {:?}", cache.get(&"foo").unwrap());
//     println!("bar: {:?}", cache.get(&"bar").unwrap());
// }

// fn generic<T>(t: T) {}

// fn generic<T: Sized>(t: T) {}

// fn generic<T: ?Sized>(t: &T) {}

// fn add_one(x: i32) -> i32 {
//     x + 1
// }
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answers = do_twice(add_one, 4);
//     println!("answer {}", answers);
// }

// fn main() {
// let list_of_numbers = vec![1, 2, 3];
//     let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

//     let list_of_numbers = vec![1, 2, 3];
//     let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

//     fn returns_closure_impl() -> impl Fn(i32) -> i32 {
//         |x| x + 1
//     }

//     fn returns_closure_box() -> Box<dyn Fn(i32) -> i32> {
//         Box::new(|x| x + 1)
//     }
// }
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }

// fn get_operation(operation: &str) -> impl Fn(i32, i32) -> i32 {
//     match operation {
//         "add" => add,
//         "multiply" => multiply,
//         _ => panic!("Invalid operation"),
//     }
// }

// fn main() {
//     let operation = get_operation("add");
//     let result = operation(2, 3);
//     println!("Result: {}", result);
// }
// 定義一個函數宏，用於計算平方

// macro_rules! square {
//     ($x:expr) => {
//         $x * $x
//     };
// }
// macro_rules! square_root {
//     ($x:expr) => {
//         ($x as f64).powf(0.5)
//     };
// }
// fn main() {
//     let x = 5;
//     let y = square!(x);
//     let c = 9;
//     let z = square_root!(c);
//     println!("The square of {} is {}", x, y); // 輸出：The square of 5 is 25
//     println!("The square_root of {} is {}", c, z); // 輸出：The square of 5 is 25
// }

pub trait Printable {
    fn print(&self);
}

use mmacro_dep::Printable;
#[derive(Debug, Printable)]
struct Example {
    value: i32,
}

use mmacro_dep::log;
#[log]
fn example_function() {
    println!("Inside example_function");
}

fn main() {
    // let example = Example { value: 42 };
    // example.print();
    example_function();
}
