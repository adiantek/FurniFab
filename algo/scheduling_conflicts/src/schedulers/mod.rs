use crate::{Instance, Schedule};

mod list_algorithm;
mod tresoldi;
mod vns;

pub use list_algorithm::list_algorithm;
pub use tresoldi::tresoldi;
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

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::error::Error;

    pub fn run_test_files<T: Scheduler + Clone>(scheduler: T) -> Result<(), Box<dyn Error>> {
        for file in std::fs::read_dir("src/test")? {
            let mut reader = std::io::BufReader::new(std::fs::File::open(file?.path())?);
            let instance = crate::serialization::deserialize(&mut reader)?;
            if !scheduler.clone().schedule(&instance).verify() {
                panic!("Invalid schedule created");
            }
        }

        Ok(())
    }
}
