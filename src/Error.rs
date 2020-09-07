use std::error::Error;
use std::fmt::{Display, Formatter};

pub const ERROR_PARSE: i32 = 1;

#[derive(Debug)]
pub struct ParseError {
    code: i32
}

impl ParseError {
    pub fn new(code: i32) -> Self {
        Self {
            code
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "NError[{},{}]", self.code, self.error_description())
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
