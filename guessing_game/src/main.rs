// use core::panic;
// use rand::Rng;
// use std::fs::File;

// //trait
// use std::io; // prelude

// fn print_hello(input: String) -> String {
//     return input;
// }

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("secret_number: {}", secret_number);
//     println!("Please input your guess");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     print!("You guessed: {guess}");
//     const STRING: &str = "guess";
//     println!("{}", STRING);
//     let array = [1, 2, 4];
//     let first_element = array[0];
//     println!("{}", first_element);

//     println!("{}", print_hello(String::from("fff")));
//     let _quit: WebEvent = WebEvent::KeyPress('q');
//     let formal = true;
//     let _greeting = if formal {
//         println!("good evening");
//     } else {
//         print!("e04");
//     };
//     let fruits = vec!["fsf", "fdsa", "dfsadf"];
//     let first = fruits.get(0);
//     println!("{:?}", first);
//     let non_index = fruits.get(4);
//     println!("{:?}", non_index);
//     // let f = File::open("test.txt").expect("no such mk file");
//     // let f = match f {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("can't not open file {:?}", error),
//     // };
//     let mut f = String::from("testt");
//     f.push_str("string");
//     let f3 = f;
//     drop(f);
//     print!("{}", f3);
// }

// enum WebEvent {
//     LoadPage,
//     UnloadPage,
//     KeyPress(char),
// }

// fn main() {
//     let s1 = give_ownership();

//     let s2 = String::from("testt");

//     let s3 = take_ownership_and_gives_back(s2);
// }

// fn give_ownership() -> String {
//     let some_string = String::from("testt");
//     return some_string;
// }

// fn take_ownership_and_gives_back(a_string: String) -> String {
//     return a_string;
// }

// fn main() {
//     let mut say = String::from("testt");
//     print_out(&mut say);
//     println!("{}", say);
// }

// fn print_out(to_print: &mut String) {
//     to_print.push_str("fff");
//     println!("{}", to_print);
// }

use rand::Rng;
use std::io;
fn main() {
    println!("猜數");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("數字為： {}", secret_number);
    println!("猜測一個數");

    loop {
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("invalid");

        //* shadow 覆蓋掉前面的guess
        //* trim去掉字串的左右兩邊空白包括
        //* parse()解析成 guess: u.i32之類的數字
        //* 如果parse成功回傳result(OK)，反之Err接著顯示expect的msg
        // let guess: u32 = guess.trim().parse().expect("please type a number");
        //? update guess 因為要確保使用者輸入的是數字
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("重新輸入");
                continue;
            }
        };

        println!("your number is : {}", guess);

        //? secret_number is u32, so we need to make sure the guess is u32 also.
        //     |     match guess.cmp(&secret_number) {
        //     |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
        //     |                 |
        //     |                 arguments to this function are incorrect
        //     |
        //     = note: expected reference `&String`
        //                found reference `&{integer}`

        match guess.cmp(&secret_number) {
            // check the number is which type.
            std::cmp::Ordering::Less => println!("too low"),
            std::cmp::Ordering::Greater => println!("too high"),
            std::cmp::Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
