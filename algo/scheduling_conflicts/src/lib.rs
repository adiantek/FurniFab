mod problem;
mod schedulers;
pub mod serialization;
mod solution;

pub use problem::*;
pub use schedulers::*;
pub use solution::*;

/// Runs the given scheduler on the instance read from stdin and writes the schedule to stdout.
/// Also writes the score to stdout.
/// Returns an error if the instance could not be read or the schedule could not be written.
///
/// # Panics
///  - If the schedule is invalid in debug mode.
pub fn run_experiment_from_stdin<T: Scheduler>(scheduler: T) -> serialization::Result<()> {
    let instance: Instance = serialization::from_stdin()?;
    let schedule = scheduler.schedule(&instance);

    debug_assert!(schedule.verify(), "Schedule is invalid: {schedule:?}");

    serialization::to_stdout(&schedule)?;
    println!("{}", schedule.calculate_score());
    Ok(())
}
