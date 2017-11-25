
//use crontab::ScheduleComponents;
//use regex::Regex;
use std::collections::HashSet;
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
    -> Result<ScheduleComponents, CrontabError> {
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

  let minutes = parse_field(fields[0], 0, 59)?;
  let hours = parse_field(fields[1], 0, 23)?;
  let days = parse_field(fields[2], 1, 31)?;
  let months = parse_field(fields[3], 1, 12)?;
  let weekdays = parse_field(fields[4], 0, 6)?;

  Ok(ScheduleComponents {
    minutes: minutes,
    hours: hours,
    days: days,
    months: months,
    weekdays: weekdays,
    seconds: Vec::new(), // TODO: Remove until implemented.
  })
}

fn parse_field(field: &str, min: u32, max: u32)
    -> Result<Vec<u32>, CrontabError> {
  let mut instances : HashSet<u32> = HashSet::new();
  let mut components = Vec::new();

  if field == "*" {
    for i in min .. (max + 1) {
      components.push(i);
    }
    return Ok(components);
  }

  for part in field.split(",") {
    let current = part.parse::<u32>()?;

    if let Some(last) = components.last() /*&& last >= current */ {
      //return CrontabError::ErrCronFormat("todo".to_string());
    }
    if current < min || current > max {
      return Err(CrontabError::ErrCronFormat(
        format!("Value outside of [{},{}] range: {}", min, max, current)));
    }
    components.push(current);
  }

  Ok(components)
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
