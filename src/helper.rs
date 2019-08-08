//! A bunch of struct, enum, etc. for helping out with coding.
//!
//! Everything very general but useful should go here.

use std::error::Error;
use std::fmt;

/// Anything using the Builder trait should be a builder object used to
/// build another object of type product.
pub trait Builder {
    /// The type you would like to build an instance of.
    type product;

    /// Creates an instance of the builder.
    fn new() -> Self;
    /// Builds an object of type product from the builder.
    ///
    /// Mostly errors from a value not being set that has no proper
    /// default.
    fn build(self) -> Result<Self::product, BuilderError>;
}

/// BuilderError is an error for the Builder trait when a build attempt
/// fails.
///
/// May be replaced or changed. Still not 100% sure how I want to handle
/// errors.
#[derive(Debug)]
pub struct BuilderError {
    error_message: &'static str,
}

impl BuilderError {
    /// Returns an instance of BuilderError
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
