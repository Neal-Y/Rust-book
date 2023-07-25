// use crate::List::{Cons, Nil};
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     use std::ptr;
//     let list_ptr = ptr::addr_of!(list);
//     println!("{:p}", list_ptr);
//     let x = 42;
//     let x_ptr = ptr::addr_of!(x);
//     println!("{:p}", x_ptr);
//     let em = 5;
//     let yyy = ptr::addr_of!(em);
//     println!("{:p}", yyy);
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use std::ops::Deref;
// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, *y);
//     // *y = *(y.deref())
// }

// fn hello(name: &str) {
//     println!("Hello {}", name);
// }
// fn main() {
//     let m = MyBox::new(String::from("neal"));
//     hello(&m);
//     //? 這裡可以看到他傳得是 &m  ->  &Mybox(String)
//     //? 之所以傳進去&MyBox(String)給吃&str的函數可以work的原因
//     //? 這時候compiler默默調用MyBox所實現的deref method 變成 &String
//     //? 也因為String有實現Deref trait所以他也調用deref method 變成 &str
//     hello(&(*m)[..]);
// }

// use std::ops::Deref;
// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data: {}", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("hello world"),
//     };
//     c.drop();
//     let d = CustomSmartPointer {
//         data: String::from("world"),
//     };

//     println!("CustomSmartPointer crated.");
// }
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));

//     let b = Cons(3, Rc::clone(&a)); //? 次數加一
//     println!("count after creating b = {}", Rc::strong_count(&a));

//     {
//         let c = Cons(4, Rc::clone(&a)); //? 次數加一
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }

//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

// fn main() {
//     let x = 5;
//     let y = &mut x;
// }

// pub trait Messenger {
//     fn send(&self, msg: &str);
// }

// pub struct LimitTracker<'a, T: 'a + Messenger> {
//     messenger: &'a T,
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T>
// where
//     T: Messenger,
// {
//     pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;
//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error:you are over your quota");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger.send("Urgent Warning : you are use 90% ");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger.send("Urgent Warning : you are use 75% ");
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::RefCell;
//     struct MockMessenger {
//         sent_messages: RefCell<Vec<String>>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, msg: &str) {
//             self.sent_messages.borrow_mut().push(String::from(msg));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

// use std::{rc::Rc, cell::RefCell};

//         limit_tracker.set_value(80);
//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
//     }
// }
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

//     *value.borrow_mut() += 10;
//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }

// use std::{cell::RefCell, rc::Rc};

// fn bang(input: Rc<RefCell<String>>) {
//     let mut result = input.borrow_mut();
//     println!("{:?}", Rc::strong_count(&input));
//     result.push_str("afdsfad");
// }
// fn main() {
//     let my_str = String::from("hello world");

//     let my_ref = Rc::new(RefCell::new(my_str));
//     bang(Rc::clone(&my_ref));
//     println!("{:?}", Rc::strong_count(&my_ref));
//     println!("{:?}", my_ref);

//     println!("{:?}", count(10));
// }

// use std::option;

// fn count(input: i32) -> i32 {
//     (1..=input).sum()
// }
// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count  = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count  = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }
//     println!("a rc count after change a = {}", Rc::strong_count(&a));
//     println!("b rc count after change a = {}", Rc::strong_count(&b));

//     println!("a next item = {:?}", a.tail());
// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// fn main() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

//     let branch = Rc::new(Node {
//         value: 5,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });

//     *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

//     println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
//     println!("branch weak count = {:?}", Rc::weak_count(&branch));
// }
