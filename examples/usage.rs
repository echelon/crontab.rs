//! Basic usage of crontab.rs

extern crate crontab;
extern crate time;

use crontab::Crontab;
use crontab::Tm;
use time::{Timespec, at_utc};

fn main() {
  let crontab = Crontab::parse("0 * * * *").expect("unparsable"); // every hour

  println!("Schedule components: {:?}\n", crontab.schedule);

  let mut timestamp = 1500001200;

  for _i in 0..10 {
    let time = to_time(timestamp);

    let next_event = crontab.find_event_after(&time).unwrap();
    let expected_event = to_time(timestamp + (60 * 60));

    assert_eq!(expected_event, next_event);

    println!("Next event: {}", expected_event.strftime("%H:%M:%S").unwrap());

    timestamp += 60 * 60;
  }
}

fn to_time(timestamp: i64) -> Tm {
  let timespec = Timespec::new(timestamp, 0);
  at_utc(timespec)
}
