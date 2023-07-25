// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4, "錯誤訊息");
//     }
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         let x = 23456;
//         assert!(smaller.can_hold(&larger), "hello this is panic msg {}", x);
//     }
// }

// pub struct Guess {
//     value: u32,
// }

// impl Guess {
//     pub fn new(value: u32) -> Guess {
//         if value < 1 {
//             panic!("guess value must be higher then 1");
//         } else if value > 100 {
//             panic!("guess value must be lower then 100");
//         }
//         return Guess { value };
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "guess value must be lower then 100")]
//     fn over_range() {
//         Guess::new(202);
//     }
// }

// fn add(x: i32, y: i32) -> i32 {
//     return x + y;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() -> Result<(), String> {
//         if add(1, 2) == 3 {
//             return Ok(());
//         } else {
//             return Err(String::from("this is an error"));
//         }
//     }
// }

pub fn prints_and_return_10(a: i32) -> i32 {
    println!("i got the value");
    return 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_should_return_10_and_success() {
        let value = prints_and_return_10(2);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_should_return_10_and_failures() {
        let value = prints_and_return_10(2);
        assert_eq!(8, value);
    }
}
