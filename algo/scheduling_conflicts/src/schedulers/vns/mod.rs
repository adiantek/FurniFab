use rand::Rng;

use crate::util::ScheduleBuilder;
use crate::{Instance, Schedule};

mod init;
mod neighborhoods;

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
    let mut schedule = neighborhood_search(init::schedule(instance));
    let mut best_score = schedule.calculate_score();

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let mut new_schedule = schedule.clone();

        for _ in 0..(instance.tasks.len() / 20).max(1) {
            let task = rng.gen_range(0..instance.tasks.len());
            let task_machine = new_schedule.get_schedule(task).map(|info| info.processor);

            new_schedule.reorganize_schedule(|machines, tardy_tasks| {
                let mut machine_fixings = Vec::with_capacity(2);

                match task_machine {
                    Some(machine) => {
                        if let Some(pos) = machines[machine].iter().position(|&id| id == task) {
                            machine_fixings.push((machine, pos));
                        }
                        machines[machine].retain(|&id| id != task);
                    }
                    None => tardy_tasks.retain(|&id| id != task),
                }

                let new_machine = rng.gen_range(0..instance.processors);
                let new_position = rng.gen_range(0..machines[new_machine].len() + 1);
                machines[new_machine].insert(new_position, task);

                match task_machine.filter(|&machine| machine == new_machine) {
                    Some(_) => machine_fixings[0].1 = new_position.min(machine_fixings[0].1),
                    None => machine_fixings.push((new_machine, new_position)),
                }

                (machine_fixings, vec![])
            });
        }

        let new_schedule = neighborhood_search(new_schedule);
        let new_score = new_schedule.calculate_score();

        if new_score > best_score {
            best_score = new_score;
            schedule = new_schedule;
        }
    }

    schedule.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedulers::test_utils::run_test_files;

    #[test]
    fn test_vns() {
        run_test_files(vns).expect("Error running tests")
    }
}
