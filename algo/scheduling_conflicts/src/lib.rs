mod problem;
mod schedulers;
pub mod serialization;
mod solution;

pub use problem::*;
pub use schedulers::*;
pub use solution::*;

pub fn run_experiment_from_stdin<T: Scheduler>(scheduler: T) -> serialization::Result<()> {
    let instance: Instance = serialization::from_stdin()?;
    serialization::to_stdout(&scheduler.schedule(&instance))
}
