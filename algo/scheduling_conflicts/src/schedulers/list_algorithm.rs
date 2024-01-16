use crate::{Instance, Schedule, ScheduleInfo, Task};
use std::collections::BTreeSet;

pub fn list_algorithm(instance: &Instance) -> Schedule {
    let mut schedule = Schedule::new(instance);
    let mut machines: BTreeSet<(u64, usize)> = BTreeSet::new();

    for machine in 0..instance.processors {
        machines.insert((0, machine));
    }

    let mut tasks: Vec<(usize, Task)> = instance.tasks.iter().copied().enumerate().collect();
    tasks.sort_unstable_by_key(|task| task.1.weight);
    tasks.reverse();

    for (id, task) in tasks {
        let mut schedule_info = None;

        for (time, machine) in &machines {
            if !schedule.in_conflict(id, *time) {
                schedule_info = Some(ScheduleInfo::new(*time, *machine));
                break;
            }
        }

        if let Some(schedule_info) = schedule_info {
            if schedule_info.start_time + task.processing_time <= instance.deadline {
                schedule.schedule(id, schedule_info);
                machines.remove(&(schedule_info.start_time, schedule_info.processor));
                machines.insert((
                    schedule_info.start_time + task.processing_time,
                    schedule_info.processor,
                ));
                continue;
            }
        }

        let non_conflict_time = schedule.available_start_time(id);

        if non_conflict_time + task.processing_time <= instance.deadline {
            let mut machine_iter = machines.iter().rev();
            let mut picked_machine = None;
            while let Some((time, machine)) = machine_iter.next_back() {
                if *time < non_conflict_time {
                    picked_machine = Some((*time, *machine));
                    break;
                }
            }

            if let Some((time, machine)) = picked_machine {
                schedule.schedule(id, ScheduleInfo::new(non_conflict_time, machine));
                machines.remove(&(time, machine));
                machines.insert((non_conflict_time + task.processing_time, machine));
            }
        }
    }

    schedule
}
