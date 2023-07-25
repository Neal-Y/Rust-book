// use std::{thread, time::Duration};
// fn main() {
//     let v = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1)); //? 一毫秒
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1)); //? 一毫秒
//     }

//     v.join().unwrap();
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 3, 4];
//     let handle = thread::spawn(move || println!("here's a vector! {:?}", v));

//     handle.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("Got:{}", received);
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("tx1:hi"),
//             String::from("tx1:from"),
//             String::from("tx1:the"),
//             String::from("tx1:thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5); //? 就是要保護的數據

//     {
//         let mut x = m.lock().unwrap();
//         *x += 1;
//     }

//     println!("{:?}", m);
// }
// use std::fmt;

// struct MyMutex<T> {
//     inner: std::sync::Mutex<T>,
// }

// impl<T> MyMutex<T> {
//     fn new(t: T) -> Self {
//         MyMutex {
//             inner: std::sync::Mutex::new(t),
//         }
//     }
// }

// impl<T: fmt::Display> fmt::Display for MyMutex<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Mutex({})", self.inner.lock().unwrap())
//     }
// }

// use std::sync::Mutex;

// fn main() {
//     let v = Mutex::new(0);

//     {
//         let mut x = v.lock().unwrap();
//         *x += 2;
//     }

//     println!("{:?}", v);
// }

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个计数器的 Mutex 对象
    let counter = Arc::new(Mutex::new(0));

    // 创建多个线程，对计数器进行增加操作
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
            println!("thread {} incremented the counter to {}", i, *count);
            thread::sleep(Duration::from_millis(1));
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数器值
    println!("final counter: {}", *counter.lock().unwrap());
}
