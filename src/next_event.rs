use scheduler::ScheduleSpec;
use time::Tm;
use times::{adv_year, adv_month, adv_day, adv_hour, adv_minute};

// TODO/FIXME: API is a bit strange.
/// Get the next time this schedule is to be executed.
pub (crate) fn calculate_next_event(times: &ScheduleSpec, time: &Tm) -> Option<Tm> {
  let mut next_time = time.clone();

  // Minute-resolution. We're always going to round up to the next minute.
  next_time.tm_sec = 0;
  adv_minute(&mut next_time);

  loop {
    match try_month(times, &mut next_time) {
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::ContinueMatching => {}, // Continue
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_day(times, &mut next_time) {
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::ContinueMatching => {}, // Continue
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_hour(times, &mut next_time) {
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::ContinueMatching => {}, // Continue
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }

    match try_minute(times, &mut next_time) {
      DateTimeMatch::Missed => continue, // Retry
      DateTimeMatch::ContinueMatching => return Some(next_time), // Uhh...
      DateTimeMatch::AnswerFound(upcoming) => return Some(upcoming),
    }
  }
}

enum DateTimeMatch {
  Missed,
  ContinueMatching,
  AnswerFound(Tm),
}

fn try_month(times: &ScheduleSpec, time: &mut Tm) -> DateTimeMatch {
  // Tm month range is [0, 11]
  // Cron months are [1, 12]
  let test_month = (time.tm_mon + 1) as u32;

  match times.months.binary_search(&test_month) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::ContinueMatching
    },
    Err(pos) => {
      if let Some(month) = times.months.get(pos) {
        // Next month. We're done.
        let mut use_time = time.clone();
        use_time.tm_mon = (month - 1) as i32;
        // Tm day range is [1, 31]
        use_time.tm_mday = times.days.get(0).unwrap().clone() as i32;
        // Tm hour range is [0, 23]
        use_time.tm_hour = times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = times.minutes.get(0).unwrap().clone() as i32;
        use_time.tm_sec = 0; // Second resolution

        DateTimeMatch::AnswerFound(use_time)

      } else {
        // Skipped beyond. Pop to last unit and use next value.
        time.tm_year = time.tm_year + 1;
        // Tm month range is [0, 11], Cron months are [1, 12]
        time.tm_mon = (times.months.get(0).unwrap().clone() - 1) as i32;
        // Tm day range is [1, 31]
        time.tm_mday = times.days.get(0).unwrap().clone() as i32;
        // Tm hour range is [0, 23]
        time.tm_hour = times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        time.tm_min = times.minutes.get(0).unwrap().clone() as i32;
        time.tm_sec = 0; // Second resolution

        DateTimeMatch::Missed
      }
    }
  }
}

fn try_day(times: &ScheduleSpec, time: &mut Tm) -> DateTimeMatch {
  match times.days.binary_search(&(time.tm_mday as u32)) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::ContinueMatching
    },
    Err(pos) => {
      if let Some(day) = times.days.get(pos) {
        // Next day. We're done.
        let mut use_time = time.clone();
        // Tm day range is [1, 31]
        use_time.tm_mday = day.clone() as i32;
        // Tm hour range is [0, 23]
        use_time.tm_hour = times.hours.get(0).unwrap().clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = times.minutes.get(0).unwrap().clone() as i32;
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

fn try_hour(times: &ScheduleSpec, time: &mut Tm) -> DateTimeMatch {
  match times.hours.binary_search(&(time.tm_hour as u32)) {
    Ok(_) => {
      // Precise month... must keep matching
      DateTimeMatch::ContinueMatching
    },
    Err(pos) => {
      if let Some(hour) = times.hours.get(pos) {
        // Next hour. We're done.
        let mut use_time = time.clone();
        // Tm hour range is [0, 23]
        use_time.tm_hour = hour.clone() as i32;
        // Tm minute range is [0, 59]
        use_time.tm_min = times.minutes.get(0).unwrap().clone() as i32;
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

fn try_minute(times: &ScheduleSpec, time: &mut Tm) -> DateTimeMatch {
  match times.minutes.binary_search(&(time.tm_min as u32)) {
    Ok(_) => {
      // DONE
      let mut use_time = time.clone();
      //use_time.tm_min = minute.clone() as i32;
      use_time.tm_sec = 0; // Second resolution
      DateTimeMatch::AnswerFound(use_time)
    },
    Err(pos) => {
      if let Some(minute) = times.minutes.get(pos) {
        // Next minute. We're done.
        let mut use_time = time.clone();
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
  use scheduler::Scheduler;
  use test_helpers::get_tm;
  use test_helpers::normal;
  use time::{Timespec, at_utc};

  fn parse_times(schedule: &str) -> ScheduleSpec {
    let schedule = Scheduler::new(schedule).ok().unwrap();
    schedule.times
  }

  #[test]
  fn every_minute() {
    let times = parse_times("* * * * *"); // every minute

    // Advances the minute
    let tm = get_tm(2001, 1, 1, 12, 0, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 1, 0)));

    // Again
    let tm = get_tm(2001, 1, 1, 12, 30, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 31, 0)));

    // Advances the hour
    let tm = get_tm(2001, 1, 1, 12, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 13, 0, 0)));

    // Advances the day
    let tm = get_tm(2001, 1, 1, 23, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 2, 0, 0, 0)));

    // Advances the month
    let tm = get_tm(2001, 1, 31, 23, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 2, 1, 0, 0, 0)));

    // Seconds get rounded up to the next minute
    let tm = get_tm(2001, 1, 1, 12, 0, 1);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2001, 1, 1, 12, 1, 0)));
  }

  #[test]
  fn every_fifteen_minutes() {
    let times = parse_times("*/15 * * * *");

    // Minute before :15 (2017-05-15 11:14)
    let tm = get_tm(2017, 5, 15, 11, 14, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 15, 0)));

    // Minute after :15 (2017-05-15 11:16)
    let tm = get_tm(2017, 5, 15, 11, 16, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 30, 0)));

    // Minute after :30 (2017-05-15 11:31)
    let tm = get_tm(2017, 5, 15, 11, 31, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 5, 15, 11, 45, 0)));

    // Minute before :00 (2017-10-15 23:59)
    let tm = get_tm(2017, 10, 15, 23, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 16, 0, 0, 0)));

    // Two minutes before New Year (2017-12-31 23:58)
    let tm = get_tm(2017, 12, 31, 23, 58, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 1, 1, 0, 0, 0)));

    // Minute before New Year (2017-12-31 23:59)
    let tm = get_tm(2017, 12, 31, 23, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn precise_date_and_time() {
    let times = parse_times("0 0 1 10 *"); // 0:00 Oct 1st

    // Minute before
    let tm = get_tm(2017, 9, 30, 23, 59, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Second before
    let tm = get_tm(2017, 9, 30, 23, 59, 59);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Month before
    let tm = get_tm(2017, 9, 1, 0, 0, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 1, 0, 0, 0)));

    // Minute after ... must wait a year!
    let tm = get_tm(2017, 10, 1, 0, 1, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 1, 0, 0, 0)));

    // Month after... must wait 11 months!
    let tm = get_tm(2017, 11, 1, 0, 0, 0);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 1, 0, 0, 0)));

    // Now with more nonzero time fields...
    // Oct 13 @ 22:45
    let times = parse_times("45 22 13 10 *");

    // Before (all time fields are nonzero)
    let tm = get_tm(2017, 7, 4, 10, 30, 1);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2017, 10, 13, 22, 45, 0)));

    // After (all time fields are nonzero)
    let tm = get_tm(2017, 11, 15, 10, 30, 15);
    let next = calculate_next_event(&times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2018, 10, 13, 22, 45, 0)));
  }

  #[test]
  fn first_of_the_month() {
    // First of the month at 0:00.
    let schedule = Scheduler::new("0 0 1 * *").ok().unwrap();
    let times = &schedule.times;

    // A minute late... advances the month.
    let tm = get_tm(2004, 1, 1, 0, 1, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // A few hours late... advances the month.
    let tm = get_tm(2004, 1, 1, 12, 59, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // Halfway through month advances the month.
    let tm = get_tm(2004, 1, 15, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2004, 2, 1, 0, 0, 0)));

    // Halfway through month at end of year advances the year.
    let tm = get_tm(2004, 12, 15, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn every_hour_in_january_and_july() {
    // Every single hour in January and July.
    let schedule = Scheduler::new("0 * * 1,7 *").ok().unwrap();
    let times = &schedule.times;

    // Last minute of December
    let tm = get_tm(2005, 12, 31, 23, 59, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2006, 1, 1, 0, 0, 0)));

    // First hour of January... advances to the next hour
    let tm = get_tm(2005, 1, 1, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 1, 1, 0, 0)));

    // Noon January 15th... advances to the next hour
    let tm = get_tm(2005, 1, 15, 12, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 1, 15, 13, 0, 0)));

    // Last minute of January... advances to July.
    let tm = get_tm(2005, 1, 31, 23, 59, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 7, 1, 0, 0, 0)));

    // First hour of July... advances to the next hour
    let tm = get_tm(2005, 7, 1, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2005, 7, 1, 1, 0, 0)));

    // Last hour of July... advances to next year's January
    let tm = get_tm(2005, 7, 31, 23, 59, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2006, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn new_years() {
    // At the New Year's ball drop.
    let schedule = Scheduler::new("0 0 1 1 *").ok().unwrap();
    let times = &schedule.times;

    // Last minute of December
    let tm = get_tm(2007, 12, 31, 23, 59, 59);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Minute zero of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Minute five of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 0, 5, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Hour one of the new year... advances to next year
    let tm = get_tm(2007, 1, 1, 1, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // Day two of the new year... advances to next year
    let tm = get_tm(2007, 1, 2, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));

    // July advances to the next year
    let tm = get_tm(2007, 7, 1, 0, 0, 0);
    let next = calculate_next_event(times, &tm).unwrap();
    expect!(normal(&next)).to(be_equal_to(get_tm(2008, 1, 1, 0, 0, 0)));
  }

  #[test]
  fn spot_check_fields_every_day() {
    // Every single day at midnight.
    let schedule = Scheduler::new("0 0 * * *").ok().unwrap();
    let times = &schedule.times;

    // 2017-01-01 00:00 UTC, a non-leap year starting on a Sunday (tm_wday=0).
    let timespec = Timespec::new(1483228800, 0);
    let mut last = at_utc(timespec);
    let mut next = last.clone();
    let mut expected = last.clone();

    // First day of 2017. (tm_year=117)
    expect!(last.tm_year).to(be_equal_to(117));
    expect!(next.tm_mon).to(be_equal_to(0));
    expect!(last.tm_yday).to(be_equal_to(0));
    expect!(next.tm_mday).to(be_equal_to(1)); // 1-indexed
    expect!(last.tm_wday).to(be_equal_to(0)); // 2017 starts on a Sunday

    for _ in 0 .. 365 {
      // We expect the next event to be the next day.
      adv_day(&mut expected);
      next = calculate_next_event(times, &last).unwrap();

      // Check expectations.
      expect!(next.tm_year).to(be_equal_to(expected.tm_year));
      expect!(next.tm_mon).to(be_equal_to(expected.tm_mon));
      expect!(next.tm_mday).to(be_equal_to(expected.tm_mday));
      expect!(next.tm_yday).to(be_equal_to(expected.tm_yday));
      expect!(next.tm_wday).to(be_equal_to(expected.tm_wday));

      // Midnight
      expect!(next.tm_hour).to(be_equal_to(0));
      expect!(next.tm_min).to(be_equal_to(0));
      expect!(next.tm_sec).to(be_equal_to(0));

      adv_day(&mut last);
    }

    // First day of 2018. (tm_year=118)
    expect!(next.tm_year).to(be_equal_to(118));
    expect!(next.tm_mon).to(be_equal_to(0));
    expect!(next.tm_yday).to(be_equal_to(0));
    expect!(next.tm_mday).to(be_equal_to(1)); // 1-indexed
    expect!(next.tm_wday).to(be_equal_to(1)); // 2018 starts on a Monday

    // The next two years aren't leap years.
    for _ in 0 .. (365 * 2) {
      // We expect the next event to be the next day.
      adv_day(&mut expected);
      next = calculate_next_event(times, &last).unwrap();

      // Check expectations.
      expect!(next.tm_year).to(be_equal_to(expected.tm_year));
      expect!(next.tm_mon).to(be_equal_to(expected.tm_mon));
      expect!(next.tm_mday).to(be_equal_to(expected.tm_mday));
      expect!(next.tm_yday).to(be_equal_to(expected.tm_yday));
      expect!(next.tm_wday).to(be_equal_to(expected.tm_wday));

      // Midnight
      expect!(next.tm_hour).to(be_equal_to(0));
      expect!(next.tm_min).to(be_equal_to(0));
      expect!(next.tm_sec).to(be_equal_to(0));

      adv_day(&mut last);
    }

    // First day of 2020.
    expect!(next.tm_year).to(be_equal_to(120));
    expect!(next.tm_mon).to(be_equal_to(0));
    expect!(next.tm_yday).to(be_equal_to(0));
    expect!(next.tm_mday).to(be_equal_to(1)); // 1-indexed
    expect!(next.tm_wday).to(be_equal_to(3)); // 2018 starts on a Wednesday

    // 2020 is a leap year
    for _ in 0 .. 366 {
      // We expect the next event to be the next day.
      adv_day(&mut expected);
      next = calculate_next_event(times, &last).unwrap();

      // Check expectations.
      expect!(next.tm_year).to(be_equal_to(expected.tm_year));
      expect!(next.tm_mon).to(be_equal_to(expected.tm_mon));
      expect!(next.tm_mday).to(be_equal_to(expected.tm_mday));
      expect!(next.tm_yday).to(be_equal_to(expected.tm_yday));
      expect!(next.tm_wday).to(be_equal_to(expected.tm_wday));

      // Midnight
      expect!(next.tm_hour).to(be_equal_to(0));
      expect!(next.tm_min).to(be_equal_to(0));
      expect!(next.tm_sec).to(be_equal_to(0));

      adv_day(&mut last);
    }

    // First day of 2021.
    expect!(next.tm_year).to(be_equal_to(121));
    expect!(next.tm_mon).to(be_equal_to(0));
    expect!(next.tm_yday).to(be_equal_to(0));
    expect!(next.tm_mday).to(be_equal_to(1)); // 1-indexed
    expect!(next.tm_wday).to(be_equal_to(5)); // 2018 starts on a Friday
  }
}
