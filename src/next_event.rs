
use scheduler::Scheduler;
use time::Tm;

/// Get the next time this schedule is to be executed.
pub fn calculate_next_event(scheduler: &Scheduler, time: &Tm) -> Option<Tm> {

  let mut next_time = time.clone();

  // Minute-resolution. We're always going to round up to the next minute.
  next_time.tm_sec = 0;
  adv_minute(&mut next_time);

  loop /* YEARS */ {

    loop /* MONTHS */ {

      match try_month(scheduler, &mut next_time) {
        DateTimeMatch::PreciseMatch => {}, // Continue
        DateTimeMatch::Missed => break, // Break out
        DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
      }

      loop /* DAYS */ {

        match try_day(scheduler, &mut next_time) {
          DateTimeMatch::PreciseMatch => {}, // Continue
          DateTimeMatch::Missed => break, // Break out
          DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
        }

        loop /* HOURS */ {
          match try_hour(scheduler, &mut next_time) {
            DateTimeMatch::PreciseMatch => {}, // Continue
            DateTimeMatch::Missed => break, // Break out
            DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
          }

          loop /* MINUTES */ {
            match try_minute(scheduler, &mut next_time) {
              DateTimeMatch::PreciseMatch => {}, // Uhh... this is braindead
              DateTimeMatch::Missed => break, // Break out
              DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
            }

            break;
          }
          break;
        }
        break;
      }
      break;
    }

    let mut use_time = next_time.clone();

    use_time.tm_year = use_time.tm_year + 1;
    // Tm month range is [0, 11], Cron months are [1, 12]
    use_time.tm_mon = (scheduler.times.months.get(0).unwrap().clone() - 1) as i32;
    // Tm day range is [1, 31]
    use_time.tm_mday = scheduler.times.days.get(0).unwrap().clone() as i32;
    // Tm hour range is [0, 23]
    use_time.tm_hour = scheduler.times.hours.get(0).unwrap().clone() as i32;
    // Tm minute range is [0, 59]
    use_time.tm_min = scheduler.times.minutes.get(0).unwrap().clone() as i32;
    use_time.tm_sec = 0; // Second resolution

    return Some(use_time);
  }

  Some(next_time)
}

enum DateTimeMatch {
  PreciseMatch,
  Missed,
  AnswerFound(Tm),
}

fn try_month(scheduler: &Scheduler, time: &mut Tm) -> DateTimeMatch {
  // Tm month range is [0, 11]
  // Cron months are [1, 12]
  let test_month = (time.tm_mon + 1) as u32;

  match scheduler.times.months.binary_search(&test_month) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::PreciseMatch
    },
    Err(pos) => {
      if let Some(month) = scheduler.times.months.get(pos) {
        // Next month. We're done.
        println!("Next month, pos: {}, month: {}", pos, month);

        let mut use_time = time.clone();
        use_time.tm_mon = (month - 1) as i32;
        // Tm day range is [1, 31]
        use_time.tm_mday = scheduler.times.days.get(0).unwrap().clone() as i32;
        // Tm hour range is [0, 23]
        use_time.tm_hour = scheduler.times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = scheduler.times.minutes.get(0).unwrap().clone() as i32;
        use_time.tm_sec = 0; // Second resolution

        DateTimeMatch::AnswerFound(use_time)

      } else {
        // Skipped beyond. Pop to last unit and use next value.
        println!("Not found, pos: {}", pos);
        DateTimeMatch::Missed
      }
    }
  }
}

fn try_day(scheduler: &Scheduler, time: &mut Tm) -> DateTimeMatch {
  match scheduler.times.days.binary_search(&(time.tm_mday as u32)) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::PreciseMatch
    },
    Err(pos) => {
      if let Some(day) = scheduler.times.days.get(pos) {
        // Next day. We're done. TODO: Days in months varies.
        let mut use_time = time.clone();
        //use_time.tm_mon = (month - 1) as i32;
        // Tm day range is [1, 31]
        use_time.tm_mday = day.clone() as i32;
        // Tm hour range is [0, 23]
        use_time.tm_hour = scheduler.times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = scheduler.times.minutes.get(0).unwrap().clone() as i32;
        use_time.tm_sec = 0; // Second resolution

        DateTimeMatch::AnswerFound(use_time)

      } else {
        // Skipped beyond. Pop to last unit and use next value.
        DateTimeMatch::Missed
      }
    }
  }
}


fn try_hour(scheduler: &Scheduler, time: &mut Tm) -> DateTimeMatch {
  match scheduler.times.hours.binary_search(&(time.tm_hour as u32)) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::PreciseMatch
    },
    Err(pos) => {
      if let Some(hour) = scheduler.times.hours.get(pos) {
        // Next day. We're done. TODO: Days in months varies.
        let mut use_time = time.clone();
        //use_time.tm_mon = (month - 1) as i32;
        // Tm day range is [1, 31]
        //use_time.tm_mday = day as i32;
        // Tm hour range is [0, 23]
        use_time.tm_hour = hour.clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = scheduler.times.minutes.get(0).unwrap().clone() as i32;
        use_time.tm_sec = 0; // Second resolution

        DateTimeMatch::AnswerFound(use_time)

      } else {
        // Skipped beyond. Pop to last unit and use next value.
        DateTimeMatch::Missed
      }
    }
  }
}


fn try_minute(scheduler: &Scheduler, time: &mut Tm) -> DateTimeMatch {
  match scheduler.times.minutes.binary_search(&(time.tm_min as u32)) {
    Ok(_) => {
      // DONE
      let mut use_time = time.clone();
      //use_time.tm_min = minute.clone() as i32;
      use_time.tm_sec = 0; // Second resolution
      DateTimeMatch::AnswerFound(use_time)
    },
    Err(pos) => {
      if let Some(minute) = scheduler.times.minutes.get(pos) {
        // Next day. We're done. TODO: Days in months varies.
        let mut use_time = time.clone();
        //use_time.tm_mon = (month - 1) as i32;
        // Tm day range is [1, 31]
        //use_time.tm_mday = day as i32;
        // Tm hour range is [0, 23]
        //use_time.tm_hour = hour as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = minute.clone() as i32;
        use_time.tm_sec = 0; // Second resolution

        DateTimeMatch::AnswerFound(use_time)

      } else {
        // Skipped beyond. Pop to last unit and use next value.
        DateTimeMatch::Missed
      }
    }
  }
}

fn adv_year(time: &mut Tm) {
  time.tm_year += 1;
}

fn adv_month(time: &mut Tm) {
  time.tm_mon += 1;
  if time.tm_mon > 11 {
    time.tm_mon = 0;
    adv_year(time);
  }
}

fn adv_day(time: &mut Tm) {
  time.tm_wday = (time.tm_wday + 1) % 7; // day of week
  time.tm_mday += 1;

  match time.tm_mon {
    0 | 2 | 4 | 6 | 7 | 9 | 11 => {
      if time.tm_mday > 31 {
        time.tm_mday = 1;
        adv_month(time);
      }
    },
    3 | 5 | 8 | 10 => {
      if time.tm_mday > 30 {
        time.tm_mday = 1;
        adv_month(time);
      }
    },
    1 => {
      // TODO: Leap years. 28 vs 29.
      if time.tm_mday > 28 {
        time.tm_mday = 1;
        adv_month(time);
      }
    },
    _ => unreachable!(),
  }
}

fn adv_hour(time: &mut Tm) {
  time.tm_hour += 1;
  if time.tm_hour > 23 {
    time.tm_hour = 0;
    adv_day(time);
  }
}

fn adv_minute(time: &mut Tm) {
  println!("{}", time.tm_min);
  time.tm_min += 1;
  println!("{}", time.tm_min);
  if time.tm_min > 59 {
    time.tm_min = 0;
    println!("{}", time.tm_min);
    adv_hour(time);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use expectest::prelude::*;

  // Get a Tm from a date. Months and days are supplied 1-indexed, but
  // the Tm struct is inconsistently 0- and 1-indexed.
  fn get_tm(year: i32,
            month: i32,
            day: i32,
            hour: i32,
            minute: i32,
            second: i32) -> Tm {

    expect!(month).to(be_greater_or_equal_to(1));
    expect!(month).to(be_less_or_equal_to(12));
    expect!(day).to(be_greater_or_equal_to(1));
    expect!(day).to(be_less_or_equal_to(31));
    expect!(hour).to(be_greater_or_equal_to(0));
    expect!(hour).to(be_less_than(24));
    expect!(minute).to(be_greater_or_equal_to(0));
    expect!(minute).to(be_less_than(60));
    expect!(second).to(be_greater_or_equal_to(0));
    expect!(second).to(be_less_or_equal_to(60)); // leap seconds

    Tm {
      tm_sec: second,
      tm_min: minute,
      tm_hour: hour,
      tm_mday: day,
      tm_mon: month.saturating_sub(1), // zero indexed
      tm_year: year.saturating_sub(1900), // Years since 1900
      tm_wday: 0, // Incorrect, but don't care
      tm_yday: 0, // Incorrect, but don't care
      tm_isdst: 0,
      tm_utcoff: 0,
      tm_nsec: 0,
    }
  }

  fn normal(time: &Tm) -> Tm {
    let mut tm = time.clone();
    tm.tm_wday = 0;
    tm.tm_yday = 0;
    tm.tm_isdst = 0;
    tm.tm_utcoff = 0;
    tm.tm_nsec= 0;
    tm
  }

  #[test]
  fn every_minute() {
    let schedule = Scheduler::new("* * * * *").ok().unwrap(); // every minute

    // Advances the minute
    let tm = get_tm(2001, 1, 1, 12, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 1, 0)));

    // Again
    let tm = get_tm(2001, 1, 1, 12, 30, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 31, 0)));

    // Advances the hour
    let tm = get_tm(2001, 1, 1, 12, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 13, 0, 0)));

    // Advances the day
    let tm = get_tm(2001, 1, 1, 23, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 2, 0, 0, 0)));

    // Advances the month
    let tm = get_tm(2001, 1, 31, 23, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 2, 1, 0, 0, 0)));

    // Seconds get rounded up to the next minute
    let tm = get_tm(2001, 1, 1, 12, 0, 1);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 1, 0)));
  }

  #[test]
  fn every_fifteen_minutes() {
    let schedule = Scheduler::new("*/15 * * * *").ok().unwrap();

    // Minute before :15 (2017-05-15 11:14)
    let tm = get_tm(2017, 5, 15, 11, 14, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 15, 0)));

    // Minute after :15 (2017-05-15 11:16)
    let tm = get_tm(2017, 5, 15, 11, 16, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 30, 0)));

    // Minute after :30 (2017-05-15 11:31)
    let tm = get_tm(2017, 5, 15, 11, 31, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 45, 0)));

    // Minute before :00 (2017-10-15 23:59)
    let tm = get_tm(2017, 10, 15, 23, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 16, 0, 0, 0)));

    // Two minutes before New Year (2017-12-31 23:58)
    let tm = get_tm(2017, 12, 31, 23, 58, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 1, 1, 0, 0, 0)));

    // Minute before New Year (2017-12-31 23:59)
    let tm = get_tm(2017, 12, 31, 23, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn precise_date_and_time() {
    let schedule = Scheduler::new("0 0 1 10 *").ok().unwrap(); // 0:00 Oct 1st

    // Minute before
    let tm = get_tm(2017, 9, 30, 23, 59, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Second before
    let tm = get_tm(2017, 9, 30, 23, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Month before
    let tm = get_tm(2017, 9, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Minute after ... must wait a year!
    let tm = get_tm(2017, 10, 1, 0, 1, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 1, 0, 0, 0)));

    // Month after... must wait 11 months!
    let tm = get_tm(2017, 11, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 1, 0, 0, 0)));

    // Now with more nonzero time fields...
    // Oct 13 @ 22:45
    let schedule = Scheduler::new("45 22 13 10 *").ok().unwrap();

    // Before (all time fields are nonzero)
    let tm = get_tm(2017, 7, 4, 10, 30, 1);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 13, 22, 45, 0)));

    // After (all time fields are nonzero)
    let tm = get_tm(2017, 11, 15, 10, 30, 15);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 13, 22, 45, 0)));
  }
}
