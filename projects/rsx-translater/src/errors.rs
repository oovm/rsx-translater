use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum RsxError {
    UnknownError,
    FormatFailed,
    ParseFailed(String),
}

pub type Result<T> = std::result::Result<T, RsxError>;

impl Error for RsxError {

}

impl Display for RsxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}



impl From<std::fmt::Error> for RsxError {
    fn from(_: std::fmt::Error) -> Self {
        Self::FormatFailed
    }
}

impl From<html_parser::Error> for RsxError {
    fn from(e: html_parser::Error) -> Self {
        Self::ParseFailed(e.to_string())
    }
}