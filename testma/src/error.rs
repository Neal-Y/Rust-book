// 在 src/error.rs 中定义错误类型
#[derive(Debug)]
pub enum NegativeNumberError {
    NegativeNumber(i32),
}
