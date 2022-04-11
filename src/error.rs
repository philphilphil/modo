use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub struct InvalidQueryError;

// #[derive(Debug)]
// struct IoError;

impl Error for InvalidQueryError {}

impl Display for InvalidQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Query.")
    }
}
