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
