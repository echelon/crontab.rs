use std::num::ParseIntError;
use std::fmt;

// TODO: These errors could use some improvement.
/// A library error.
#[derive(Debug)]
pub enum CrontabError {
  /// Error parsing the crontab schedule.
  ErrCronFormat(String),
  /// Error parsing an integer in a crontab schedule.
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
