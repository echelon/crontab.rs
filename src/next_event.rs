use scheduler::Scheduler;
use time::Tm;
use times::{adv_year, adv_month, adv_day, adv_hour, adv_minute};

/// Get the next time this schedule is to be executed.
pub fn calculate_next_event(scheduler: &Scheduler, time: &Tm) -> Option<Tm> {

  let mut next_time = time.clone();

  println!("Schedule: {:?}", scheduler.times);

  // Minute-resolution. We're always going to round up to the next minute.
  next_time.tm_sec = 0;
  adv_minute(&mut next_time);

  loop {
    match try_month(scheduler, &mut next_time) {
      DateTimeMatch::PreciseMatch => {}, // Continue
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_day(scheduler, &mut next_time) {
      DateTimeMatch::PreciseMatch => {}, // Continue
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_hour(scheduler, &mut next_time) {
      DateTimeMatch::PreciseMatch => {}, // Continue
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_minute(scheduler, &mut next_time) {
      DateTimeMatch::PreciseMatch => break, // Uhh... this is braindead
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }
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

        time.tm_year = time.tm_year + 1;
        // Tm month range is [0, 11], Cron months are [1, 12]
        time.tm_mon = (scheduler.times.months.get(0).unwrap().clone() - 1) as i32;
        // Tm day range is [1, 31]
        time.tm_mday = scheduler.times.days.get(0).unwrap().clone() as i32;
        // Tm hour range is [0, 23]
        time.tm_hour = scheduler.times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        time.tm_min = scheduler.times.minutes.get(0).unwrap().clone() as i32;
        time.tm_sec = 0; // Second resolution

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
        time.tm_mday = 1; // Reset day (1-indexed)
        time.tm_hour = 0; // Reset hour
        time.tm_min = 0; // Reset minute
        time.tm_sec = 0; // Reset second
        adv_month(time);
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
        time.tm_hour = 0; // Reset hour
        time.tm_min = 0; // Reset minute
        time.tm_sec = 0; // Reset second
        adv_day(time);
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
        time.tm_min = 0; // Reset minute
        time.tm_sec = 0; // Reset second
        adv_hour(time);
        DateTimeMatch::Missed
      }
    }
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

  #[test]
  fn first_of_the_month() {
    // First of the month at 0:00.
    let schedule = Scheduler::new("0 0 1 * *").ok().unwrap();

    // A minute late... advances the month.
    let tm = get_tm(2004, 1, 1, 0, 1, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // A few hours late... advances the month.
    let tm = get_tm(2004, 1, 1, 12, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // Halfway through month advances the month.
    let tm = get_tm(2004, 1, 15, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // Halfway through month at end of year advances the year.
    let tm = get_tm(2004, 12, 15, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn every_hour_in_january_and_july() {
    // Every single hour in January and July.
    let schedule = Scheduler::new("0 * * 1,7 *").ok().unwrap();

    // Last minute of December
    let tm = get_tm(2005, 12, 31, 23, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2006, 1, 1, 0, 0, 0)));

    // First hour of January... advances to the next hour
    let tm = get_tm(2005, 1, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 1, 1, 0, 0)));

    // Noon January 15th... advances to the next hour
    let tm = get_tm(2005, 1, 15, 12, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 15, 13, 0, 0)));

    // Last minute of January... advances to July.
    let tm = get_tm(2005, 1, 31, 23, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 7, 1, 0, 0, 0)));

    // First hour of July... advances to the next hour
    let tm = get_tm(2005, 7, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 7, 1, 1, 0, 0)));

    // Last hour of July... advances to next year's January
    let tm = get_tm(2005, 7, 31, 23, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2006, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn new_years() {
    // At the New Year's ball drop.
    let schedule = Scheduler::new("0 0 1 1 *").ok().unwrap();

    // Last minute of December
    let tm = get_tm(2007, 12, 31, 23, 59, 59);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Minute zero of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Minute five of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 0, 5, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Hour one of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 1, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Day two of the new year... advances to next year
    let tm = get_tm(2007, 1, 2, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // July advances to the next year
    let tm = get_tm(2007, 7, 1, 0, 0, 0);
    let next = calculate_next_event(&schedule, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));
  }
}
