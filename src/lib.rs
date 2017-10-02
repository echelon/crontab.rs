//! Crontab.rs is a library for parsing cron schedule expressions.

extern crate time;
extern crate regex;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

mod error;
mod next_event;
mod scheduler;