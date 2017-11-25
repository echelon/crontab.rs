use error::CrontabError;

/// The components of a crontab schedule.
#[derive(Clone, Debug, Default)]
pub struct ScheduleComponents {
  /// Minutes in the schedule.
  pub minutes: Vec<u32>,
  /// Hours in the schedule.
  pub hours: Vec<u32>,
  /// Days in the schedule.
  pub days: Vec<u32>,
  /// Months in the schedule.
  pub months: Vec<u32>,
  /// Weekdays in the schedule.
  pub weekdays: Vec<u32>,
  /// Seconds in the schedule.
  /// TODO: Mark deprecated until implemented.
  pub seconds: Vec<u32>,
}

pub (crate) fn parse_cron(schedule: &str)
    -> Result<ScheduleComponents, CrontabError> {
  let fields : Vec<&str> = schedule.trim()
      .split_whitespace()
      .collect();

  if fields.len() != 5 {
    return Err(CrontabError::ErrCronFormat(
      format!("invalid format: {}", schedule)));
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
  let mut components = Vec::new();

  if field == "*" {
    for i in min .. (max + 1) {
      components.push(i);
    }
    return Ok(components);
  }

  for part in field.split(",") {
    let current = part.parse::<u32>()?;

    if let Some(last) = components.last() {
      if last >= &current {
        return Err(CrontabError::ErrCronFormat("todo".to_string())); // TODO
      }
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

  #[test]
  fn bare_wildcards() {
    let parsed = parse_cron("* * * * *").unwrap();

    expect!(parsed.minutes).to(be_equal_to((0..60).collect::<Vec<u32>>()));
    expect!(parsed.hours).to(be_equal_to((0..24).collect::<Vec<u32>>()));
    expect!(parsed.days).to(be_equal_to((1..32).collect::<Vec<u32>>()));
    expect!(parsed.months).to(be_equal_to((1..13).collect::<Vec<u32>>()));
    expect!(parsed.weekdays).to(be_equal_to((0..7).collect::<Vec<u32>>()));
  }

  #[test]
  fn specified_minutes() {
    let parsed = parse_cron("0 * * * *").unwrap();
    expect!(parsed.minutes).to(be_equal_to(vec![0]));

    let parsed = parse_cron("5,10,15 * * * *").unwrap();
    expect!(parsed.minutes).to(be_equal_to(vec![5,10,15]));

    let parsed = parse_cron("59 * * * *").unwrap();
    expect!(parsed.minutes).to(be_equal_to(vec![59]));

    // Outside range
    let result = parse_cron("60 * * * *");
    expect!(result).to(be_err());

    let result = parse_cron("-1 * * * *");
    expect!(result).to(be_err());
  }

  #[test]
  fn specified_hours() {
    let parsed = parse_cron("* 0 * * *").unwrap();
    expect!(parsed.hours).to(be_equal_to(vec![0]));

    let parsed = parse_cron("* 1,12,20 * * *").unwrap();
    expect!(parsed.hours).to(be_equal_to(vec![1, 12, 20]));

    let parsed = parse_cron("* 23 * * *").unwrap();
    expect!(parsed.hours).to(be_equal_to(vec![23]));

    // Outside range
    let result = parse_cron("* 24 * * *");
    expect!(result).to(be_err());

    let result = parse_cron("* -1 * * *");
    expect!(result).to(be_err());
  }

  #[test]
  fn specified_values_must_be_in_order() {
    expect!(parse_cron("1,2,3 * * * *")).to(be_ok());
    expect!(parse_cron("3,2,1 * * * *")).to(be_err());
  }

  #[test]
  fn specified_values_must_be_unique() {
    expect!(parse_cron("1,2,3 * * * *")).to(be_ok());
    expect!(parse_cron("1,1,1 * * * *")).to(be_err());
  }
}
