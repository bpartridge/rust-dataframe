use arrow::error::ArrowError;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum DataFrameError {
    MemoryError(String),
    ParseError(String),
    ComputeError(String),
    DivideByZero,
    IoError(String),
    NoneError,
    ArrowError(String),
}

impl From<ArrowError> for DataFrameError {
    fn from(error: ArrowError) -> Self {
        DataFrameError::ArrowError(error.to_string())
    }
}

impl From<::std::io::Error> for DataFrameError {
    fn from(error: ::std::io::Error) -> Self {
        DataFrameError::IoError(error.to_string())
    }
}

impl From<std::option::NoneError> for DataFrameError {
    fn from(_error: ::std::option::NoneError) -> Self {
        DataFrameError::NoneError
    }
}

impl From<std::str::Utf8Error> for DataFrameError {
    fn from(error: ::std::str::Utf8Error) -> Self {
        DataFrameError::ParseError(error.to_string())
    }
}

pub type Result<T> = ::std::result::Result<T, DataFrameError>;
