//* 所有權轉移(ownership transferred)

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s1);
// } //? s1's ownership was moved to s2, so we can not to access by freed variables. because the "OWNERSHIP"
// error[E0382]: borrow of moved value: `s1`
//  --> src/main.rs:4:20
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 |     println!("{}", s1);
//   |                    ^^ value borrowed here after move
//   |

// fn main() {
//     let s1 = give_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back_ownership(s2);
// }

// fn give_ownership() -> String {
//     let some_string = String::from("hello");

//     return some_string;
// }

// fn takes_and_gives_back_ownership(a_string: String) -> String {
//     return a_string;
// }

//? s1 呼叫 give_ownership()把give_ownership裡所產生的字串的"所有權"傳出去到 s1
//? s2 生成一個String並在s3那行呼叫一個 takes_and_gives_back_ownership()把s2的所有權轉移到函數裡面再將函數裡面那個所有權回傳給s3

//* 引用(Reference)

// fn main() {
//     let s1 = String::from("hello world");

//     let len = calculate_length(&s1);

//     println!("the length of {} string length is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

//? &引用並不會轉移所有權
//? 不可以修改&的變數，除非加上mut

// fn main() {
//     let mut s = String::from("hello world");

//     let s1 = &mut s;

//     let s2 = &mut s;

//     println!("{},{}", s1, s2);
// }
//? error[E0499]: cannot borrow `s` as mutable more than once at a time
//   --> src/main.rs:62:14
//    |
// 60 |     let s1 = &mut s;
//    |              ------ first mutable borrow occurs here
// 61 |
// 62 |     let s2 = &mut s;
//    |              ^^^^^^ second mutable borrow occurs here
// 63 |
// 64 |     println!("{},{}", s1, s2);
//    |                       -- first borrow later used here

// For more information about this error, try `rustc --explain E0499`.

//? 防止同樣的變數“同時”有不同的&Reference要去更改 ->數據競爭

//* 切片

// fn main() {
//     let mut s = String::from("hello world");
//     let count = first_word(&s);

//     println!("{}", count);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     return s.len();
// }
//? 這樣設計存在bug因為回傳的usize不聯動，所以不太好

// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..=10];
//     println!("{}", world);
// }
//? 改寫

// fn main() {
//     let mut s = String::from("hello world");
//     let count = first_word(&s);
//     s.clear();
//     println!("{}", count);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     return &s[..];
// }
//? 不可以發生同時又不變又可變的
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//    --> src/main.rs:113:5
//     |
// 112 |     let count = first_word(&s);
//     |                            -- immutable borrow occurs here
// 113 |     s.clear();
//     |     ^^^^^^^^^ mutable borrow occurs here
// 114 |     println!("{}", count);
//     |                    ----- immutable borrow later used here

// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `ownership` due to previous error

// fn main() {
//     let my_string = String::from(" hello world");
//     let word_index = first_word(&my_string[..]);

//     let my_string_literal = " hello world";
//     let word_index_literal = first_word(my_string_literal);

//     println!("{} {}", word_index, word_index_literal);
// }

// fn first_word(s: &str) -> &str {
//     let some_string = s.as_bytes();//?把"hello world"轉成byte code
//     for (i, &item) in some_string.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     return &s[..];
// }

//? 優化
// fn first_word(s: &String) -> &str {
//     let words: Vec<&str> = s.split_whitespace().collect();
//     return words[0];
// }
// fn main() {
//     let s = String::from("  hello world");
//     let count = first_word(&s);
//     println!("{}", count);
// }
fn main() {
    let v = vec![1, 2, 3];
    // let mut iter = v.iter();

    // match iter.next() {
    //     Some(x) => println!("Next element is {:?}", x),
    //     None => println!("No more element"),
    // }
    for x in v.iter() {
        println!("Next element is {:?}", x)
    }
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..=2];
    println!("{:?}", slice);
}
