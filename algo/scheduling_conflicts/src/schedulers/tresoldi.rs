use ahash::HashMap;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::thread_rng;

use crate::util::TaskWithId;
use crate::{Instance, Schedule, ScheduleInfo};

const ITERATIONS: usize = 10;

struct ScheduleBuilder<'a> {
    instance: &'a Instance,
    score: u64,
    tasks: Vec<TaskWithId>,
    matrix: Vec<Vec<Option<usize>>>,
    scheduled: HashMap<usize, (usize, u64)>,
}

impl<'a> ScheduleBuilder<'a> {
    fn empty(instance: &'a Instance) -> Self {
        Self {
            instance,
            score: Default::default(),
            tasks: Default::default(),
            matrix: Default::default(),
            scheduled: Default::default(),
        }
    }

    fn random(instance: &'a Instance, rng: &mut ThreadRng) -> Self {
        let mut tasks: Vec<TaskWithId> = instance.tasks.clone().into_iter().enumerate().collect();
        tasks.shuffle(rng);
        Self {
            instance,
            score: 0,
            tasks,
            matrix: vec![vec![None; instance.processors]; instance.deadline as usize],
            scheduled: Default::default(),
        }
    }

    fn greedy_insert(&mut self) -> bool {
        let mut change = false;

        for time in 0..self.matrix.len() as u64 {
            for machine in 0..self.matrix[0].len() {
                if self.matrix[time as usize][machine].is_none() {
                    for task in &self.tasks {
                        if !self.scheduled.contains_key(&task.0)
                            && self.check_time(time, machine, task)
                            && self.check_conflicts(task, time)
                        {
                            for instant in time..(time + task.1.processing_time) {
                                self.matrix[instant as usize][machine] = Some(task.0);
                            }
                            self.score += task.1.weight;
                            self.scheduled.insert(task.0, (machine, time));
                            change = true;
                            break;
                        }
                    }
                }
            }
        }

        change
    }

    fn local_search(&mut self) -> bool {
        let mut change = false;

        for &old in &self.tasks {
            if let Some((machine, time)) = self.scheduled.get(&old.0).copied() {
                for &task in &self.tasks {
                    if self.scheduled.contains_key(&task.0) {
                        continue;
                    }

                    if (task.1.weight > old.1.weight
                        || (task.1.weight == old.1.weight
                            && task.1.processing_time < old.1.processing_time))
                        && self.check_hole(&old, &task)
                        && self.check_conflicts(&task, time)
                    {
                        for instant in time..(time + old.1.processing_time) {
                            self.matrix[instant as usize][machine] = None;
                        }

                        for instant in time..(time + task.1.processing_time) {
                            self.matrix[instant as usize][machine] = Some(task.0);
                        }

                        self.score = self.score - old.1.weight + task.1.weight;
                        self.scheduled.remove(&old.0);
                        self.scheduled.insert(task.0, (machine, time));
                        change = true;
                        break;
                    }
                }
            }
        }

        change
    }

    fn compact(&mut self) -> bool {
        let mut change = false;

        for &task in &self.tasks {
            if let Some((machine, time)) = self.scheduled.get(&task.0).copied() {
                let mut best_machine = machine;
                let mut best_time = time;

                for machine in 0..self.matrix[0].len() {
                    let mut free_time = 0;

                    for time in 0..best_time + task.1.processing_time - 1 {
                        if self.matrix[time as usize][machine].is_none() {
                            free_time += 1;

                            if free_time == task.1.processing_time
                                && self.check_conflicts(&task, time - free_time + 1)
                            {
                                best_time = time - free_time + 1;
                                best_machine = machine;
                            }
                        } else {
                            free_time = 0;
                        }
                    }
                }

                if best_time < time {
                    for instant in time..(time + task.1.processing_time) {
                        self.matrix[instant as usize][machine] = None;
                    }

                    for instant in best_time..(best_time + task.1.processing_time) {
                        self.matrix[instant as usize][best_machine] = Some(task.0);
                    }

                    self.scheduled.insert(task.0, (best_machine, best_time));
                    change = true;
                }
            }
        }

        change
    }

    fn check_time(&self, time: u64, machine: usize, task: &TaskWithId) -> bool {
        if time + task.1.processing_time > self.matrix.len() as u64 {
            return false;
        }

        for instant in time..(time + task.1.processing_time) {
            if self.matrix[instant as usize][machine].is_some() {
                return false;
            }
        }

        true
    }

    fn check_conflicts(&self, task: &TaskWithId, time: u64) -> bool {
        for &conflict in self.instance.graph.conflicts(task.0) {
            if let Some(&(_, other_time)) = self.scheduled.get(&conflict) {
                let other_task = self.instance.tasks[conflict];
                if time < other_time + other_task.processing_time
                    && other_time < time + task.1.processing_time
                {
                    return false;
                }
            }
        }

        true
    }

    fn check_hole(&self, task: &TaskWithId, new_task: &TaskWithId) -> bool {
        if task.1.processing_time >= new_task.1.processing_time {
            return true;
        }

        let &(machine, time) = self.scheduled.get(&task.0).unwrap();

        if time + new_task.1.processing_time > self.matrix.len() as u64 {
            return false;
        }

        let start = (time + task.1.processing_time) as usize;
        let end = (time + new_task.1.processing_time) as usize;
        for instant in &self.matrix[start..end] {
            if instant[machine].is_some() {
                return false;
            }
        }

        true
    }
}

impl<'a> From<ScheduleBuilder<'a>> for Schedule<'a> {
    fn from(value: ScheduleBuilder<'a>) -> Self {
        let mut schedule = Schedule::new(value.instance);

        for (task, (machine, time)) in value.scheduled {
            schedule.schedule(task, ScheduleInfo::new(time, machine));
        }

        schedule
    }
}

pub fn tresoldi(instance: &Instance) -> Schedule {
    let mut rng = thread_rng();
    let mut best_solution = ScheduleBuilder::empty(instance);

    for _ in 0..ITERATIONS {
        let mut solution = ScheduleBuilder::random(instance, &mut rng);

        loop {
            let mut change = solution.greedy_insert();
            change |= solution.local_search();
            change |= solution.compact();

            if !change {
                break;
            }
        }

        if solution.score > best_solution.score {
            best_solution = solution;
        }
    }

    best_solution.into()
}
