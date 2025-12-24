use core::fmt;

#[derive(Debug)]
pub enum AppError{
    InvalidUrl,
    NotFound,
    Internal,
}

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

pub type AppResult<T> = Result<T,AppError>;