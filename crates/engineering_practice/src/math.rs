use crate::errors::AppError;

pub fn divide(left: i32, right: i32) -> Result<i32, AppError> {
    if right == 0 {
        Err(AppError::DivideByZero)
    } else {
        Ok(left / right)
    }
}
