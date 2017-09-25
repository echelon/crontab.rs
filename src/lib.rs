
extern crate time;
extern crate regex;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

mod error;
mod next_event;
mod scheduler;

#[cfg(test)]
mod tests {
  /*use cron_rs::Scheduler;

  #[test]
  fn test_cron() {
    let ret = Scheduler::new("* /5 * * * *").unwrap();
    let ret = Scheduler::new("* * /5 * * *").unwrap();
    let ret = Scheduler::new("0 20 * * *").unwrap();

    println!("Result: {:?}", ret);

  }*/
}