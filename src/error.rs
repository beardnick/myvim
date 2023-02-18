use std::error;
use std::fmt::{Display, Formatter, write};

#[derive(Debug)]
pub enum PlugError {
    InvalidRepo,
    CloneFailed(String),
}

impl Display for PlugError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl error::Error for PlugError{
}
