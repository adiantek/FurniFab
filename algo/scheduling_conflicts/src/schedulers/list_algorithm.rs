use crate::util::{weighted_task_comparator, ScheduleBuilder, TaskWithId};
use crate::{Instance, Schedule};

pub fn list_algorithm(instance: &Instance) -> Schedule {
    let mut schedule = ScheduleBuilder::new(instance);
    let mut machines = schedule.new_machine_free_times();

    let mut tasks: Vec<TaskWithId> = instance.tasks.iter().copied().enumerate().collect();
    tasks.sort_unstable_by(weighted_task_comparator);

    for (id, task) in tasks {
        let mut found_machine = None;

        for &machine in &machines {
            if !schedule.in_conflict(id, machine.free_time) {
                found_machine = Some(machine);
                break;
            }
        }

        if let Some(mut machine) = found_machine {
            if machine.free_time + task.processing_time <= instance.deadline {
                schedule.schedule(id, machine.free_time, machine.id);
                machines.remove(&machine);
                machine.free_time += task.processing_time;
                machines.insert(machine);
                continue;
            }
        }

        let mut machine_with_time = None;
        for &machine in &machines {
            if let Some(time) = schedule.calculate_non_conflict_time(id, machine.free_time) {
                machine_with_time = Some((time, machine));
                break;
            }
        }

        if let Some((time, mut machine)) = machine_with_time {
            schedule.schedule(id, time, machine.id);
            machines.remove(&machine);
            machine.free_time = time + task.processing_time;
            machines.insert(machine);
        }
    }

    schedule.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedulers::test_utils::run_test_files;

    #[test]
    fn test_list_algorithm() {
        run_test_files(list_algorithm).expect("Error running tests")
    }
}
