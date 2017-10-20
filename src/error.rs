use std::num::ParseIntError;
use std::fmt;

pub enum CrontabError {
  ErrCronFormat(String),
  ErrParseInt(ParseIntError),
}

impl From<ParseIntError> for CrontabError {
  fn from(err: ParseIntError) -> CrontabError {
    CrontabError::ErrParseInt(err)
  }
}

impl fmt::Display for CrontabError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &CrontabError::ErrCronFormat(ref x) => write!(f, "<ErrCronFormat> {:?}", x),
      &CrontabError::ErrParseInt(ref e) => write!(f, "<ErrParseInt> {:?}", e),
    }
  }
}
