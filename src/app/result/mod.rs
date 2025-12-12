use crate::app::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;
