//! Crontab.rs is a library for parsing cron schedule expressions.

#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]

extern crate time;
extern crate regex;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

#[cfg(test)]
mod test_helpers;

mod error;
mod crontab;
mod next_event;
mod scheduler;
mod times;

// Exports
pub use crontab::Crontab;
pub use crontab::ScheduleComponents;

// Re-exports.
pub use time::Tm;
