mod error;
mod math;
use math::square;
fn main() {
    match square(2) {
        Ok(res) => println!("2 的平方是：{}", res),
        Err(e) => match e {
            error::NegativeNumberError::NegativeNumber(x) => {
                println!("错误：输入的数字 {} 为负数", x)
            }
        },
    }

    match square(-2) {
        Ok(res) => println!("2 的平方是：{}", res),
        Err(e) => match e {
            error::NegativeNumberError::NegativeNumber(x) => {
                println!("错误：输入的数字 {} 为负数", x)
            }
        },
    }
    let x = 4;
    let test_to_catch_x = |z: i32| z == x;
    let b = 4;
    println!("{:?}", assert!(test_to_catch_x(b)));
}
