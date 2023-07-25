// 在 src/math.rs 中引用错误类型并定义计算平方的函数
use crate::error::NegativeNumberError;

pub fn square(x: i32) -> Result<i32, NegativeNumberError> {
    if x < 0 {
        Err(NegativeNumberError::NegativeNumber(x))
    } else {
        Ok(x * x)
    }
}
