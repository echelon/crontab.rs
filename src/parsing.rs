
//use crontab::ScheduleComponents;
//use regex::Regex;
use error::CrontabError;

/// The components of a crontab schedule.
#[derive(Clone, Debug, Default)]
pub struct ScheduleComponents {
  /// Months in the schedule.
  pub months: Vec<u32>,
  /// Days in the schedule.
  pub days: Vec<u32>,
  /// Weekdays in the schedule.
  pub weekdays: Vec<u32>,
  /// Hours in the schedule.
  pub hours: Vec<u32>,
  /// Minutes in the schedule.
  pub minutes: Vec<u32>,
  /// Seconds in the schedule.
  pub seconds: Vec<u32>,
}



pub (crate) fn parse_cron(schedule: &str)
    -> Result<(), CrontabError> {
  // Regex is taken from 'cron-rs'
  /*let regex = Regex::new(r"^\s*((\*(/\d+)?)|[0-9-,/]+)(\s+((\*(/\d+)?)|[0-9-,/]+)){4,5}\s*$")
      .expect("Regex must parse");

  if !regex.is_match(schedule) {
    return Err(CrontabError::ErrCronFormat(format!("invalid format: {}", schedule)))
  }*/

  let fields : Vec<&str> = schedule.trim().split_whitespace().collect();

  if fields.len() != 5 {
    return Err(CrontabError::ErrCronFormat(format!("invalid format: {}", schedule)))
  }

  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;
  use expectest::prelude::*;

  #[test]
  fn parse_correct_size() {
    expect!(parse_cron("")).to(be_err());
    expect!(parse_cron("* * * *")).to(be_err());
    expect!(parse_cron("* * * * * *")).to(be_err());
    expect!(parse_cron("* * * * *")).to(be_ok());

    // Leading and trailing spaces
    expect!(parse_cron("   ")).to(be_err());
    expect!(parse_cron("  * * * *  ")).to(be_err());
    expect!(parse_cron("  * * * * * *  ")).to(be_err());
    expect!(parse_cron("  * * * * *  ")).to(be_ok());

    // Newlines, tabs
    expect!(parse_cron("\n\t")).to(be_err());
    expect!(parse_cron("\n\t* * * *\n\t")).to(be_err());
    expect!(parse_cron("\n\t* * * * * *\n\t")).to(be_err());
    expect!(parse_cron("\n\t* * * * *\n\t")).to(be_ok());
  }
}
