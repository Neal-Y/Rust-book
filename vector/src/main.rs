// fn main() {
//     let v: Vec<i32> = Vec::new();
//     let mut v2 = vec![1, 3, 4];
//     v2.push(6);
//     let answer = v2[1];
//     v2[1] = 2;
//     println!("{:?}{}", v2, answer);
//     for i in &mut v2 {
//         *i += 2;
//     }
//     for i in &v2 {
//         println!("{i}");
//     }
//     #[derive(Debug)]
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
//     println!("{:#?}", row);
// }

//? String

// fn main() {
//     let s = "fsdfsaf";
//     let s2 = "fsdfsaf";
//     let sum = s.to_owned() + s2; //? 這邊要用to_owned()的原因是因爲要把&str這個借用準備一個擁有所有權的記憶體位置
//                                  //? 這是存儲在堆上並且具有比原始借用參考更長的生存期。這允許您擁有數據並且在不受借用限制的情況下操作它
//     println!("{}.{}.{}", s, s2, sum);

// }

// fn main() {
//     let mut s = String::from("fsdf");
//     let s2 = String::from("fsdf");
//     s.push_str(&s2);
//     let sum = s + &s2;
//     println!("{}", s); //? 這邊不行是因為s把他的所有權轉移到'+'的裡面
//     println!("{}", s2);
//     println!("{}", sum);
// }
//* format! 方法
// fn main() {
// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");
// let s = format!("{}-{}-{}", s1, s2, s3);
// println!("{}", s);
// let str_var = "安安你好啊";

// for b in str_var.as_bytes() {
//     print!("{}, ", b);
// }

// for c in str_var.chars() {
//     print!("{}, ", c);
// }
// let x = "你好我是可可";
// let mut uy = x.chars();
// match uy.next() {
//     Some(x) => println!("{}", x),
//     None => (),
// }
// println!("{}", x);
// loop {
//     match uy.next() {
//         Some(x) => print!("{}", x),
//         _ => break,
//     }
// }
// }

// fn main() {
//     let s1 = "我你好好";
//     let count = s1.len();
//     println!("{}", count);
//     let greeting = "你好世界";
//     let greeting_start = &greeting[0..9];
//     println!("{}", greeting_start);
// }
// fn main() {
//     let greeting: Vec<char> = "你好世界".chars().collect();
//     println!("{}", &greeting[..].iter().collect::<String>());
//     // for i in greeting.iter() {
//     //     println!("{:?}", i);
//     // }
//     println!("{}", &greeting[0]);
//     let s = "你好世界";
//     let chars: Vec<char> = s.chars().collect();
//     println!("{}", &chars[0..=1].iter().collect::<String>());
// }

//? hashmap

// use std::collections::HashMap;

// fn main() {
//     let mut scores = std::collections::HashMap::new();
//     scores.insert(String::from("blue"), 10);
//     println!("{:?}", scores);
// }

// use std::collections::HashMap;

// fn main() {
//     let teams = vec![String::from("Blue"), String::from("Yellow")];
//     let initial_score = vec![10, 50];

//     let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
//     println!("{:?}", scores);
// }

// fn main() {
//     let field_name = String::from("field_name");
//     let field_value = String::from("field_value");
//     let mut map = HashMap::new();
//     // map.insert(field_name, field_value); //? 所有權轉移

//     map.insert(&field_name, &field_value); //? 用引用解決
//     println!("{}:{}", field_name, field_value);
// }

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Bluefff"), 50);
//     scores.insert(String::from("f"), 2);

//     for (k, v) in &scores {
//         println!("{}:{}", k, v);
//     }
// }

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);
//     match score {
//         Some(s) => println!("{}", s),
//         None => println!("not exist"),
//     }
// }

//? update hashmap

use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);

//     scores.entry(String::from("Yellow")).or_insert(50);
//     scores.entry(String::from("Blue")).or_insert(20);

//     println!("{:?}", scores);
// }

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
