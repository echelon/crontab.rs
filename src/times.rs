use time::Tm;

pub (crate) fn adv_year(time: &mut Tm) {
  // TODO: Handle leap years effectively, as they'll effect tm_yday and tm_mday.
  time.tm_year += 1;
}

pub (crate) fn adv_month(time: &mut Tm) {
  time.tm_mon += 1;
  if time.tm_mon > 11 {
    time.tm_mon = 0;
    adv_year(time);
  }
}

pub (crate) fn adv_day(time: &mut Tm) {
  time.tm_wday = (time.tm_wday + 1) % 7; // day of week
  time.tm_yday = (time.tm_yday + 1) % 366; // day of year

  time.tm_mday += 1; // day of month

  let is_leap_year = {
    let year = time.tm_year + 1900;
    if year % 400 == 0
        || (year % 4 == 0 && year % 100 != 0) {
      true
    } else {
      false
    }
  };

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
      let mdays = if is_leap_year { 29 } else { 28 };

      if time.tm_mday > mdays {
        time.tm_mday = 1;
        adv_month(time);
      }
    },
    _ => unreachable!(),
  }
}

pub (crate) fn adv_hour(time: &mut Tm) {
  time.tm_hour += 1;
  if time.tm_hour > 23 {
    time.tm_hour = 0;
    adv_day(time);
  }
}

pub (crate) fn adv_minute(time: &mut Tm) {
  time.tm_min += 1;
  if time.tm_min > 59 {
    time.tm_min = 0;
    adv_hour(time);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use expectest::prelude::*;
  use test_helpers::get_tm;
  use test_helpers::normal;

  #[test]
  pub fn test_adv_year() {
    let mut tm = get_tm(2017, 10, 6, 12, 24, 0);
    adv_year(&mut tm);
    expect!(normal(&tm)).to(be_equal_to(get_tm(2018, 10, 6, 12, 24, 0)));
  }

  #[test]
  pub fn test_adv_month() {
    // January
    let mut tm = get_tm(2017, 1, 1, 12, 0, 0);
    adv_month(&mut tm);
    expect!(normal(&tm)).to(be_equal_to(get_tm(2017, 2, 1, 12, 0, 0)));

    // December
    let mut tm = get_tm(2017, 12, 1, 0, 0, 0);
    adv_month(&mut tm);
    expect!(normal(&tm)).to(be_equal_to(get_tm(2018, 1, 1, 0, 0, 0)));
  }

  use time::at_utc;
  use time::Timespec;
  #[test]
  pub fn test_mday() {
    // 2017-01-01 00:00 UTC, a non-leap year starting on a Sunday (tm_wday=0).
    let timespec = Timespec::new(1483228800, 0);
    let mut tm = at_utc(timespec);

    // 2017 to 2019 are not leap years
    let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // 2017 is (tm_year=117)
    for tm_year in 117 .. 120 {
      expect!(tm.tm_year).to(be_equal_to(tm_year));

      for days_in_month in days_in_months.iter() {
        let bound = days_in_month + 1; // 1-indexed
        for expected_day in 1..bound {
          expect!(tm.tm_mday).to(be_equal_to(expected_day));
          adv_day(&mut tm);
        }
      }
    }

    expect!(tm.tm_year).to(be_equal_to(120));

    // 2020 is a leap-year
    let days_in_months = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for days_in_month in days_in_months.iter() {
      let bound = days_in_month + 1; // 1-indexed
      for expected_day in 1..bound {
        expect!(tm.tm_mday).to(be_equal_to(expected_day));
        adv_day(&mut tm);
      }
    }

    expect!(tm.tm_year).to(be_equal_to(121));
  }
}
