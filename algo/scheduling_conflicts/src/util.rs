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
    schedule: Schedule<'a>,
    machines: Vec<Vec<usize>>,
    tardy_tasks: Vec<usize>,
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

    /// Returns the schedule for a task.
    pub fn get_schedule(&self, task: usize) -> Option<&ScheduleInfo> {
        self.schedule.get_schedule(task)
    }

    /// Marks a task as tardy.
    pub fn tardy(&mut self, task: usize) {
        self.tardy_tasks.push(task);
    }

    /// Returns the number of machines.
    pub fn machines_len(&self) -> usize {
        self.machines.len()
    }

    /// Returns the number of tasks in a machine.
    pub fn machine_tasks_len(&self, machine: usize) -> usize {
        self.machines[machine].len()
    }

    /// Returns the number of tardy tasks.
    pub fn tardy_len(&self) -> usize {
        self.tardy_tasks.len()
    }

    /// Calculates the score of the schedule.
    pub fn calculate_score(&self) -> u64 {
        self.schedule.calculate_score()
    }

    /// Creates an ordered set of machines with order of free time.
    pub fn new_machine_free_times(&mut self) -> BTreeSet<Machine> {
        self.machines
            .iter()
            .enumerate()
            .map(|(id, tasks)| {
                let free_time = tasks
                    .last()
                    .and_then(|&task| self.schedule.get_schedule(task).map(|info| (task, info)))
                    .map(|(task, info)| info.start_time + self.instance.tasks[task].processing_time)
                    .unwrap_or_default();
                Machine::with_free_time(id, free_time)
            })
            .collect()
    }

    /// Check if the given task with the given start time is in conflict with another task.
    pub fn in_conflict(&self, task: usize, start_time: u64) -> bool {
        self.schedule.in_conflict(task, start_time)
    }

    /// Calculates first available time for a task that is not in conflict with other tasks.
    /// It returns None if there is no available time within deadline.
    pub fn calculate_non_conflict_time(&self, task: usize, minimum_time: u64) -> Option<u64> {
        let processing_time = self.instance.tasks[task].processing_time;
        let mut times: Vec<_> = self
            .instance
            .graph
            .conflicts(task)
            .iter()
            .filter_map(|&other| {
                self.schedule
                    .get_schedule(other)
                    .map(|info| info.start_time + self.instance.tasks[other].processing_time)
            })
            .filter(|&time| time >= minimum_time)
            .filter(|&time| time + processing_time <= self.instance.deadline)
            .filter(|&time| !self.schedule.in_conflict(task, time))
            .collect();
        times.sort_unstable();
        times.first().copied()
    }

    /// Reorganizes the schedule using the given operations.
    /// It removes the tasks that are changed and fixes the machines and tardy tasks.
    /// The op function should return a tuple with machine id, index, and tardy tasks.
    pub fn reorganize_schedule<F>(&mut self, op: F)
    where
        F: FnOnce(&mut [Vec<usize>], &mut Vec<usize>) -> (Vec<(usize, usize)>, Vec<usize>),
    {
        let (machines, tardy) = op(&mut self.machines, &mut self.tardy_tasks);

        for task in tardy {
            self.schedule.remove_schedule(task);
        }

        for (machine, index) in &machines {
            for &task in &self.machines[*machine][*index..] {
                self.schedule.remove_schedule(task);
            }
        }

        for (machine, index) in machines {
            self.fix_machine(machine, index);
        }

        self.fix_tardy();
    }

    fn fix_machine(&mut self, machine: usize, start_index: usize) {
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

        self.machines[machine].retain(|&id| self.schedule.get_schedule(id).is_some());
    }

    fn fix_tardy(&mut self) {
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
