// fn main() {
//     panic!("Hello, world!");
// }

use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let _result_of_file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_crate) => file_crate,
                Err(error_crate) => panic!("error creating file{:?}", error_crate),
            },
            other => panic!("error opening file {:?}", other),
        },
    };
    let f = File::open("hello.txt").unwrap_or_else(|error_open| {
        if error_open.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error_crate| {
                panic!("Error creating file: {:?}", error_open);
            })
        } else {
            panic!("Error opening file: {:?}", error_open);
        }
    });
    let f = File::open("hello.txt").expect("hello");
}

// use std::{fs::File, io, io::Read};

// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     let f = File::open(file_name);
//     let mut f = match f {
//         Ok(file) => file,
//         Err(_) => match File::create(file_name) {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         },
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn main() {
//     let text = read_username_from_file("hello.txt");
//     println!("{:?}", text);
// }

// use std::{fs::File, io, io::Read};

// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open(file_name)?.read_to_string(&mut s)?;
//     return Ok(s);
// }

// fn main() {
//     let text = read_username_from_file("hello.txt");
//     println!("{:?}", text);
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 {
            panic!("guess value must be between 0 and 100")
        }
        return Guess { value };
    }
    pub fn getter(&self) -> i32 {
        return self.value;
    }
}

fn main() {
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue,
        };

        let guess = Guess::new(guess);
        //? 這樣就是把驗證這個數字的方法打包成一個constructor
        //? 只要是生成這個數字就都要通過驗證
    }
}

fn main() {
    // let add = |x: i32, y: i32| -> i32 { x + y };
    // let multiply = |x: i32, y: i32| -> i32 { x * y };

    let operation = get_operation("add");
    println!("1 + 2 = {}", operation(1, 2));

    let operation = get_operation("multiply");
    println!("3 * 4 = {}", operation(3, 4));
}

fn get_operation(name: &str) -> fn(i32, i32) -> i32 {
    match name {
        "add" => |x: i32, y: i32| -> i32 { x + y },
        "multiply" => |x: i32, y: i32| -> i32 { x * y },
        _ => |_, _| -> i32 { panic!("Invalid operation name") },
    }
}
