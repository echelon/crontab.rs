use error::CrontabError;
use next_event::calculate_next_event;
use scheduler::{TimeSpec, Scheduler};

/// Represents a Crontab schedule.
/// (Currently this is just an opaque type over 'cron_rs'. In the future, this
/// will contain its own parsing logic.)
pub struct Crontab {
  times: TimeSpec,
}

impl Crontab {

  /// Parse a crontab schedule.
  pub fn parse(crontab_schedule: &str) -> Result<Crontab, CrontabError> {
    let scheduler = Scheduler::new(crontab_schedule)?;

    Ok(Crontab {
      times: scheduler.times.clone(),
    })
  }

  // TODO/FIXME: API is a bit strange.
  /// Find when the next event will take place.
  pub fn find_next_event(&self, time: &Tm) -> Option<Tm> {
    calculate_next_event(self.times, time)
  }
}
