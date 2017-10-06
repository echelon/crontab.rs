use time::Tm;

pub fn adv_year(time: &mut Tm) {
  time.tm_year += 1;
}

pub fn adv_month(time: &mut Tm) {
  time.tm_mon += 1;
  if time.tm_mon > 11 {
    time.tm_mon = 0;
    adv_year(time);
  }
}

pub fn adv_day(time: &mut Tm) {
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

pub fn adv_hour(time: &mut Tm) {
  time.tm_hour += 1;
  if time.tm_hour > 23 {
    time.tm_hour = 0;
    adv_day(time);
  }
}

pub fn adv_minute(time: &mut Tm) {
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
}
