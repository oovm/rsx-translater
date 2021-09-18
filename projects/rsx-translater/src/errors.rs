use std::fmt::Error;

#[derive(Debug, Clone)]
pub enum RsxError {
    UnknownError,
    FormatFailed,
    ParseFailed(String),
}

pub type Result<T> = std::result::Result<T, RsxError>;


impl From<std::fmt::Error> for RsxError {
    fn from(_: Error) -> Self {
        Self::FormatFailed
    }
}

impl From<html_parser::Error> for RsxError {
    fn from(e: html_parser::Error) -> Self {
        Self::ParseFailed(e.to_string())
    }
}