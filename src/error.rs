use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = axum::response::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<8} - {self:?}", "INTO RES");

        let status = match self {
            Error::LoginFail => StatusCode::UNAUTHORIZED,
        };

        (status, format!("Error: {self}")).into_response()
    }
}
