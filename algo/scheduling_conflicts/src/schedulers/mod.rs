use crate::{Instance, Schedule};

mod list_algorithm;
mod vns;

pub use list_algorithm::list_algorithm;
pub use vns::vns;

/// A scheduler. Schedules the tasks of an instance.
pub trait Scheduler {
    fn schedule(self, instance: &Instance) -> Schedule;
}

impl<T: FnOnce(&Instance) -> Schedule> Scheduler for T {
    fn schedule(self, instance: &Instance) -> Schedule {
        self(instance)
    }
}
