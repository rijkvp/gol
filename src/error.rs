use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error: {0}, at row {1}, col {2}")]
    ParseError(String, usize, usize),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error)
}