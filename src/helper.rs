use std::error::Error;
use std::fmt;

pub trait Builder {
    type product;

    fn new() -> Self;
    fn build(self) -> Result<Self::product, BuilderError>;
}

#[derive(Debug)]
pub struct BuilderError {
    error_message: &'static str,
}

impl BuilderError {
    pub fn new(error_message: &'static str) -> BuilderError {
        BuilderError { error_message }
    }
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BUILD ERROR: {}", self.error_message)
    }
}

impl Error for BuilderError {
    fn description(&self) -> &str {
        "An error building an object occured."
    }

    // add side stuff in future
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
