//! The Scheduler code is taken from cron_rs 0.1.6 as found on
//! https://github.com/stormgbs/cron-rs/blob/master/src/scheduler.rs
//! on 2017-09-16. The code is licensed under the MIT per the license file,
//! https://github.com/stormgbs/cron-rs/blob/master/LICENSE
//!
//! Copyright (c) 2015 Ray Solomon <raybsolomon@gmail.com>
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of
//! this software and associated documentation files (the "Software"), to deal in
//! the Software without restriction, including without limitation the rights to
//! use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
//! of the Software, and to permit persons to whom the Software is furnished to do
//! so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

// TODO: Ditch this code.

#![allow(non_snake_case)]
#![allow(unused_variables, dead_code)]

use std::collections::{HashSet, HashMap};

use time;
use regex::Regex;

use crontab::ScheduleSpec;
use error::CrontabError::ErrCronFormat;
use error::CrontabError;

pub (crate) type SchedulerResult<'a> = Result<Scheduler<'a>, CrontabError>;


#[derive(Debug)]
pub (crate) struct Scheduler<'a> {
  seconds: &'a str,
  minutes: &'a str,
  hours: &'a str,
  days: &'a str,
  months: &'a str,
  weekdays: &'a str,

  timeFiledsLength: usize,

  pub timePoints: HashMap<&'a str, HashSet<u32>>,

  pub times: ScheduleSpec,

  re: Regex,
}

impl<'a> Scheduler<'a> {
  pub fn new(intervals: &'a str) -> SchedulerResult {
    let reRes = Regex::new(r"^\s*((\*(/\d+)?)|[0-9-,/]+)(\s+((\*(/\d+)?)|[0-9-,/]+)){4,5}\s*$");

    match reRes {
      Ok(re) => {
        if !re.is_match(intervals) {
          return Err(ErrCronFormat(format!("invalid format: {}", intervals)));
        }

        let timeFileds: Vec<&str> = intervals.split_whitespace().collect();
        let timeFiledsLength = timeFileds.len();

        if timeFiledsLength != 5 && timeFiledsLength != 6 {
          return Err(ErrCronFormat(format!("length of itervals should be 5 or 6, \
                                                but got {}",
                                           timeFiledsLength)));
        }

        let mut sec = "";
        let mut startIndex: usize = 0;

        if timeFiledsLength == 6 {
          sec = timeFileds[0].clone();
          startIndex = 1;
        }

        let mut sch = Scheduler {
          seconds: sec,
          minutes: timeFileds[startIndex],
          hours: timeFileds[startIndex + 1],
          days: timeFileds[startIndex + 2],
          months: timeFileds[startIndex + 3],
          weekdays: timeFileds[startIndex + 4],
          timeFiledsLength: timeFiledsLength,
          timePoints: HashMap::new(),
          times: ScheduleSpec::default(),
          re: re,
        };

        try!(sch.parse_time_fields().map_err(|e| ErrCronFormat(e.to_string())));
        Ok(sch)
      }
      Err(e) => Err(ErrCronFormat(e.to_string())),
    }
  }

  pub fn parse_time_fields(&mut self) -> Result<(), CrontabError> {
    if self.seconds != "" {
      self.timePoints.insert("seconds", try!(parse_intervals_field(self.seconds, 0, 59)));
    } else {
      self.timePoints.insert("seconds", [0].iter().cloned().collect::<HashSet<u32>>());
    }

    self.timePoints.insert("minutes", try!(parse_intervals_field(self.minutes, 0, 59)));
    self.timePoints.insert("hours", try!(parse_intervals_field(self.hours, 0, 23)));
    self.timePoints.insert("days", try!(parse_intervals_field(self.days, 1, 31)));
    self.timePoints.insert("months", try!(parse_intervals_field(self.months, 1, 12)));
    self.timePoints.insert("weekdays", try!(parse_intervals_field(self.weekdays, 0, 6)));

    let get_sorted = |timePoints: &HashMap<&'a str, HashSet<u32>>, label: &str| {
      let values = timePoints.get(label).unwrap();
      let mut values : Vec<u32> = values.into_iter()
          .map(|u| u.clone())
          .collect();
      values.sort();
      values
    };


    self.times.months = get_sorted(&self.timePoints, "months");
    self.times.days = get_sorted(&self.timePoints, "days");
    self.times.weekdays = get_sorted(&self.timePoints, "weekdays");
    self.times.hours = get_sorted(&self.timePoints, "hours");
    self.times.minutes = get_sorted(&self.timePoints, "minutes");
    self.times.seconds = get_sorted(&self.timePoints, "seconds");

    Ok(())
  }

  pub fn is_time_up(&self, t: &time::Tm) -> bool {
    let (second, minute, hour, day, month, weekday) = (t.tm_sec as u32,
                                                       t.tm_min as u32,
                                                       t.tm_hour as u32,
                                                       t.tm_mday as u32,
                                                       t.tm_mon as u32,
                                                       t.tm_wday as u32);

    let isSecond = self.timePoints.get("seconds").unwrap().contains(&second);
    let isLeft = self.timePoints.get("minutes").unwrap().contains(&minute) &&
                 self.timePoints.get("hours").unwrap().contains(&hour) &&
                 self.timePoints.get("days").unwrap().contains(&day) &&
                 self.timePoints.get("months").unwrap().contains(&month) &&
                 self.timePoints.get("weekdays").unwrap().contains(&weekday);

    if self.timeFiledsLength == 5 {
      isLeft
    } else {
      isSecond && isLeft
    }
  }
}

fn parse_intervals_field(inter: &str, min: u32, max: u32) -> Result<HashSet<u32>, CrontabError> {
  let mut points = HashSet::new();
  let parts: Vec<&str> = inter.split(",").collect();

  for part in parts {
    let x: Vec<&str> = part.split("/").collect();
    let y: Vec<&str> = x[0].split("-").collect();

    let mut _min = min;
    let mut _max = max;
    let mut step = 1u32;

    let (xLen, yLen) = (x.len(), y.len());

    if xLen == 1 && yLen == 1 {
      if y[0] != "*" {
        _min = try!(y[0].parse::<u32>());
        _max = _min;
      }
    } else if xLen == 1 && yLen == 2 {
      _min = try!(y[0].parse::<u32>());
      _max = try!(y[1].parse::<u32>());

    } else if xLen == 2 && yLen == 1 && x[0] == "*" {
      step = try!(x[1].parse::<u32>());

    } else {
      return Err(ErrCronFormat(String::from(part)));
    }

    for i in (_min.._max + 1).filter(|x| x % step == 0).collect::<Vec<u32>>() {
      points.insert(i);
    }
  }

  Ok(points)
}

#[test]
fn test_parse_intervals() {
  assert!(Scheduler::new("*/2 1-8,11 * * *").is_ok());
  assert!(Scheduler::new("0 */2 1-8,11 * * *").is_ok());
  assert!(Scheduler::new("*/2 1-4,16,11,17 * * *").is_ok());
  assert!(Scheduler::new("05 */2 1-8,11 * * * *").is_err());
  assert!(Scheduler::new("05 */ 1-8,11 * * *").is_err());
}
