use std::num::ParseIntError;
use std::fmt;

// TODO: Rename TaskError
pub enum Error {
  ErrCronFormat(String),
  ErrParseInt(ParseIntError),
}

impl From<ParseIntError> for Error {
  fn from(err: ParseIntError) -> Error {
    Error::ErrParseInt(err)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::ErrCronFormat(ref x) => write!(f, "<ErrCronFormat> {:?}", x),
      &Error::ErrParseInt(ref e) => write!(f, "<ErrParseInt> {:?}", e),
    }
  }
}
