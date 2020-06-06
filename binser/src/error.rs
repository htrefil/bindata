use std::fmt::{self, Display, Formatter};
use std::io::{self, ErrorKind};

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

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(ErrorKind::InvalidData, self.to_string())
    }
}
