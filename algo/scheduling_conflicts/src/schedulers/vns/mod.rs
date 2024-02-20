mod init;
mod neighborhoods;

use crate::util::ScheduleBuilder;
use crate::{Instance, Schedule};

fn neighborhood_search(mut schedule: ScheduleBuilder) -> ScheduleBuilder {
    let factories = [
        neighborhoods::swap_single_machine,
        neighborhoods::move_single_machine,
        neighborhoods::swap_two_machines,
        neighborhoods::move_two_machines,
        neighborhoods::replace_with_tardy,
        neighborhoods::add_tardy,
    ];

    let mut k = 0;

    while k < factories.len() {
        let mut best_score = schedule.calculate_score();
        let mut best_schedule = None;

        for schedule in factories[k](&schedule) {
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
