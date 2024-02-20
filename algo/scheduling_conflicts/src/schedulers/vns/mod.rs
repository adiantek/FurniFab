mod init;
mod neighborhoods;

use crate::util::ScheduleBuilder;
use crate::{Instance, Schedule};

fn neighborhood_search(mut schedule: ScheduleBuilder) -> ScheduleBuilder {
    let neighborhood_amount = neighborhoods::factories().len();

    let mut k = 0;

    while k < neighborhood_amount {
        let mut best_score = schedule.calculate_score();
        let mut best_schedule = None;

        for schedule in neighborhoods::factories()[k](&schedule) {
            let score = schedule.calculate_score();
            if score > best_score {
                best_score = score;
                best_schedule = Some(schedule);
            }
        }

        if let Some(best_schedule) = best_schedule {
            schedule = best_schedule;
            k = 0;
        } else {
            k += 1;
        }
    }

    schedule
}

pub fn vns(instance: &Instance) -> Schedule {
    let schedule = neighborhood_search(init::schedule(instance));
    schedule.into()
}
