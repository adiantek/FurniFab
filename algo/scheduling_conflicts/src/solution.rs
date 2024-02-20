use crate::Instance;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schedule info for a task. Contains the start time and processor of the task.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq)]
pub struct ScheduleInfo {
    pub processor: usize,
    pub start_time: u64,
}

impl ScheduleInfo {
    /// Creates new schedule info.
    pub fn new(start_time: u64, processor: usize) -> Self {
        ScheduleInfo {
            start_time,
            processor,
        }
    }
}

/// A schedule. Contains the schedule info for every task.
#[derive(Clone, Debug, Serialize)]
pub struct Schedule<'a> {
    #[serde(skip)]
    instance: &'a Instance,
    schedule: Vec<Option<ScheduleInfo>>,
}

impl<'a> Schedule<'a> {
    /// Creates a new schedule.
    pub fn new(instance: &'a Instance) -> Self {
        Schedule {
            instance,
            schedule: vec![None; instance.tasks.len()],
        }
    }

    /// Schedule info for a task.
    pub fn schedule(&mut self, task: usize, schedule_info: ScheduleInfo) {
        self.schedule[task] = Some(schedule_info);
    }

    /// Removes the schedule info for a task.
    pub fn remove_schedule(&mut self, task: usize) {
        self.schedule[task] = None;
    }

    /// Get the schedule info for a task.
    pub fn get_schedule(&self, task: usize) -> Option<&ScheduleInfo> {
        self.schedule[task].as_ref()
    }

    /// Check if the given task with the given start time is in conflict with another task.
    pub fn in_conflict(&self, task: usize, start_time: u64) -> bool {
        self.instance.graph.conflicts(task).iter().any(|&other| {
            if let Some(schedule_info) = self.schedule[other] {
                let task = &self.instance.tasks[task];
                let other = &self.instance.tasks[other];
                start_time < schedule_info.start_time + other.processing_time
                    && schedule_info.start_time < start_time + task.processing_time
            } else {
                false
            }
        })
    }

    /// Calculates the score of the schedule.
    pub fn calculate_score(&self) -> u64 {
        let mut score = 0;
        for (schedule_info, task) in self.schedule.iter().zip(&self.instance.tasks) {
            if let Some(schedule_info) = schedule_info {
                if schedule_info.start_time + task.processing_time <= self.instance.deadline {
                    score += task.weight;
                }
            }
        }
        score
    }

    /// Checks if schedule is valid.
    pub fn verify(&self) -> bool {
        let mut machines = vec![BTreeMap::new(); self.instance.processors];

        for (id, schedule_info) in self.schedule.iter().enumerate() {
            if let Some(schedule_info) = schedule_info {
                let machine = &mut machines[schedule_info.processor];

                if machine.contains_key(&schedule_info.start_time) {
                    return false;
                }

                machine.insert(schedule_info.start_time, id);
            }
        }

        for machine in machines {
            let mut last_end_time = 0;
            for (start_time, task) in machine {
                if start_time < last_end_time {
                    return false;
                }

                last_end_time = start_time + self.instance.tasks[task].processing_time;
            }
        }

        for (id, schedule_info) in self.schedule.iter().enumerate() {
            if let Some(schedule_info) = schedule_info {
                if self.in_conflict(id, schedule_info.start_time) {
                    return false;
                }
            }
        }

        true
    }
}
