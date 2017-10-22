use error::CrontabError;
use next_event::calculate_next_event;
use scheduler::Scheduler;
use time::Tm;

/// Represents a crontab schedule.
pub struct Crontab {
  /// The components parsed from a crontab schedule.
  pub schedule: ScheduleComponents,
}

/// The components of a crontab schedule.
#[derive(Clone, Debug, Default)]
pub struct ScheduleComponents {
  /// Months in the schedule.
  pub months: Vec<u32>,
  /// Days in the schedule.
  pub days: Vec<u32>,
  /// Weekdays in the schedule.
  pub weekdays: Vec<u32>,
  /// Hours in the schedule.
  pub hours: Vec<u32>,
  /// Minutes in the schedule.
  pub minutes: Vec<u32>,
  /// Seconds in the schedule.
  pub seconds: Vec<u32>,
}

impl Crontab {

  /// Parse a crontab schedule.
  pub fn parse(crontab_schedule: &str) -> Result<Crontab, CrontabError> {
    let scheduler = Scheduler::new(crontab_schedule)?;

    Ok(Crontab {
      schedule: scheduler.times,
    })
  }

  // TODO/FIXME: Optional API is a bit strange. Get rid of the Option wrapper.
  /// Find when the next event will take place.
  pub fn find_next_event(&self, time: &Tm) -> Option<Tm> {
    calculate_next_event(&self.schedule, time)
  }
}
