use actix_web::{ResponseError};
use derive_more::derive::{Display, Error};

#[derive(Display, Error, Debug)]
pub enum AppError{
    #[display("Token Not found")]
    TokenNotFound,
    #[display("Internal Error Occurred")]
    InternalError
}

impl ResponseError for AppError{}