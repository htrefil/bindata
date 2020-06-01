use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Overflow,
    InvalidVariant,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Overflow => write!(f, "Out of bounds read"),
            Error::InvalidVariant => write!(f, "Invalid variant of an enum"),
        }
    }
}
