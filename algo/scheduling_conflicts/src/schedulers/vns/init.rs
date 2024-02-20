use crate::util::{weighted_task_comparator, ScheduleBuilder, TaskWithId};
use crate::Instance;

/// Returns an initial schedule, machine schedules and tardy tasks.
/// Simple list scheduling algorithm.
pub fn schedule(instance: &Instance) -> ScheduleBuilder {
    let mut schedule = ScheduleBuilder::new(instance);
    let mut machines = schedule.new_machine_free_times();

    let mut tasks: Vec<TaskWithId> = instance.tasks.iter().copied().enumerate().collect();
    tasks.sort_unstable_by(weighted_task_comparator);

    for task in tasks {
        let mut machine = machines.pop_first().expect("No available machines");

        let time = if schedule.in_conflict(task.0, machine.free_time) {
            schedule.calculate_non_conflict_time(task.0, machine.free_time)
        } else if machine.free_time + task.1.processing_time <= instance.deadline {
            Some(machine.free_time)
        } else {
            None
        };

        if let Some(time) = time {
            schedule.schedule(task.0, time, machine.id);
            machine.free_time = time + task.1.processing_time;
        } else {
            schedule.tardy(task.0);
        }

        machines.insert(machine);
    }

    schedule
}
