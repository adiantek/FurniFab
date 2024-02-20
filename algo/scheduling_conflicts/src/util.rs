use crate::{Instance, Schedule, ScheduleInfo, Task};
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// A task with its id.
pub type TaskWithId = (usize, Task);

/// A machine is a resource that can be used to process a task.
/// It's ordered by free time.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Machine {
    pub id: usize,
    pub free_time: u64,
}

impl Machine {
    /// Creates a new machine with free time 0.
    pub fn new(id: usize) -> Self {
        Self { id, free_time: 0 }
    }

    fn with_free_time(id: usize, free_time: u64) -> Self {
        Self { id, free_time }
    }
}

impl PartialOrd<Self> for Machine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Machine {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.free_time.cmp(&other.free_time) {
            Ordering::Equal => self.id.cmp(&other.id),
            order => order,
        }
    }
}

/// Compares two tasks by their weight and processing time.
pub fn weighted_task_comparator(a: &TaskWithId, b: &TaskWithId) -> Ordering {
    (a.1.processing_time * b.1.weight).cmp(&(b.1.processing_time * a.1.weight))
}

/// A builder for creating a schedule.
/// It's used to schedule tasks on machines with utility methods.
#[derive(Clone, Debug)]
pub struct ScheduleBuilder<'a> {
    instance: &'a Instance,
    // TODO: temporary pubs
    pub schedule: Schedule<'a>,
    pub machines: Vec<Vec<usize>>,
    pub tardy_tasks: Vec<usize>,
}

impl<'a> ScheduleBuilder<'a> {
    /// Creates a new schedule builder.
    pub fn new(instance: &'a Instance) -> Self {
        Self {
            instance,
            schedule: Schedule::new(instance),
            machines: vec![Vec::new(); instance.processors],
            tardy_tasks: Vec::new(),
        }
    }

    /// Schedules a task on a machine at a given time.
    pub fn schedule(&mut self, task: usize, time: u64, machine: usize) {
        let info = ScheduleInfo::new(time, machine);
        self.schedule.schedule(task, info);
        self.machines[machine].push(task);
    }

    /// Marks a task as tardy.
    pub fn tardy(&mut self, task: usize) {
        self.tardy_tasks.push(task);
    }

    /// Calculates the score of the schedule.
    pub fn calculate_score(&self) -> u64 {
        self.schedule.calculate_score()
    }

    /// Creates an ordered set of machines with order of free time.
    pub fn new_machine_free_times(&mut self) -> BTreeSet<Machine> {
        // tODO: remove mut, refactor
        let mut machines = BTreeSet::new();
        for (id, tasks) in self.machines.iter().enumerate() {
            machines.insert(Machine::with_free_time(
                id,
                tasks
                    .last()
                    .and_then(|&task| self.schedule.get_schedule(task).as_mut().map(|x| (task, x)))
                    .map(|(x, y)| y.start_time + self.instance.tasks[x].processing_time)
                    .unwrap_or(0),
            ));
        }
        machines
    }

    /// Check if the given task with the given start time is in conflict with another task.
    pub fn in_conflict(&self, task: usize, start_time: u64) -> bool {
        self.schedule.in_conflict(task, start_time)
    }

    /// Calculates first available time for a task that is not in conflict with other tasks.
    /// It returns None if there is no available time within deadline.
    pub fn calculate_non_conflict_time(&self, task: usize, minimum_time: u64) -> Option<u64> {
        // tODO: refactor
        let mut times: Vec<_> = self
            .instance
            .graph
            .conflicts(task)
            .iter()
            .filter_map(|&other| {
                if let Some(info) = self.schedule.get_schedule2(other) {
                    let time = info.start_time + self.instance.tasks[other].processing_time;
                    if time > minimum_time {
                        Some(time)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        times.sort_unstable();
        times.into_iter().find(|&time| {
            !self.schedule.in_conflict(task, time)
                && self.instance.deadline >= time + self.instance.tasks[task].processing_time
        })
    }

    /// TODO: pub
    pub fn fix_machine(&mut self, machine: usize, start_index: usize) {
        let mut free_time = if start_index == 0 {
            0
        } else {
            let task = self.machines[machine][start_index - 1];
            self.schedule
                .get_schedule(task)
                .map(|info| info.start_time + self.instance.tasks[task].processing_time)
                .unwrap_or_default()
        };

        for &task in &self.machines[machine][start_index..] {
            let processing_time = self.instance.tasks[task].processing_time;
            let time = if self.schedule.in_conflict(task, free_time) {
                self.calculate_non_conflict_time(task, free_time)
            } else if free_time + processing_time <= self.instance.deadline {
                Some(free_time)
            } else {
                None
            };

            if let Some(time) = time {
                let info = ScheduleInfo::new(time, machine);
                self.schedule.schedule(task, info);
                free_time = time + processing_time;
            } else {
                self.tardy_tasks.push(task);
            }
        }

        self.machines[machine] = self.machines[machine] // TODO: find better way
            .iter()
            .filter(|&&id| self.schedule.get_schedule(id).is_some())
            .copied()
            .collect();
    }

    /// TODO: fix pub
    pub fn fix_tardy(&mut self) {
        self.tardy_tasks.sort_unstable_by(|&a, &b| {
            weighted_task_comparator(&(a, self.instance.tasks[a]), &(b, self.instance.tasks[b]))
        });

        let mut machines = self.new_machine_free_times();
        let mut tasks = Vec::new();

        std::mem::swap(&mut self.tardy_tasks, &mut tasks);

        for task in tasks {
            let mut machine = machines.pop_first().expect("No available machines");

            let time = if self.in_conflict(task, machine.free_time) {
                self.calculate_non_conflict_time(task, machine.free_time)
            } else if machine.free_time + self.instance.tasks[task].processing_time
                <= self.instance.deadline
            {
                Some(machine.free_time)
            } else {
                None
            };

            if let Some(time) = time {
                self.schedule(task, time, machine.id);
                machine.free_time = time + self.instance.tasks[task].processing_time;
            } else {
                self.tardy(task);
            }

            machines.insert(machine);
        }
    }
}

impl<'a> From<ScheduleBuilder<'a>> for Schedule<'a> {
    fn from(builder: ScheduleBuilder<'a>) -> Schedule<'a> {
        builder.schedule
    }
}
