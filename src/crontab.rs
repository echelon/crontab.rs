use error::CrontabError;
use next_event::calculate_next_event;
use scheduler::Scheduler;
use time::Tm;

/// Represents a crontab schedule.
pub struct Crontab {
  pub schedule: ScheduleComponents,
}

/// The components of a crontab schedule.
#[derive(Clone, Debug, Default)]
pub struct ScheduleComponents {
  pub months: Vec<u32>,
  pub days: Vec<u32>,
  pub weekdays: Vec<u32>,
  pub hours: Vec<u32>,
  pub minutes: Vec<u32>,
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

  // TODO/FIXME: API is a bit strange.
  /// Find when the next event will take place.
  pub fn find_next_event(&self, time: &Tm) -> Option<Tm> {
    calculate_next_event(&self.schedule, time)
  }
}
