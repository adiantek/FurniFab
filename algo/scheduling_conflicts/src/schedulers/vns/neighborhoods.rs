use crate::util::ScheduleBuilder;

/// Neighborhood trait represents a neighborhood of a schedule.
/// It's used to generate new schedules from a given schedule.
pub type Neighborhood<'a, 'b> = dyn Iterator<Item = ScheduleBuilder<'a>> + 'b;

/// Neighborhood that swaps two tasks on the same machine.
pub struct SwapSingleMachine<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of SwapSingleMachine neighborhood.
pub fn swap_single_machine<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(SwapSingleMachine {
        schedule,
        machine: 0,
        i: 0,
        j: 1,
    })
}

impl<'a, 'b> Iterator for SwapSingleMachine<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.machine < self.schedule.machines_len() {
            while self.i + 1 < self.schedule.machine_tasks_len(self.machine) {
                if self.j < self.schedule.machine_tasks_len(self.machine) {
                    let mut builder = self.schedule.clone();

                    builder.reorganize_schedule(|machines, _| {
                        machines[self.machine].swap(self.i, self.j);
                        (vec![(self.machine, self.i)], vec![])
                    });

                    self.j += 1;

                    return Some(builder);
                }
                self.i += 1;
            }
            self.machine += 1;
        }
        None
    }
}

/// Neighborhood that moves task on the same machine.
pub struct MoveSingleMachine<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of MoveSingleMachine neighborhood.
pub fn move_single_machine<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(MoveSingleMachine {
        schedule,
        machine: 0,
        i: 0,
        j: 1,
    })
}

impl<'a, 'b> Iterator for MoveSingleMachine<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.machine < self.schedule.machines_len() {
            while self.i + 1 < self.schedule.machine_tasks_len(self.machine) {
                if self.j < self.schedule.machine_tasks_len(self.machine) {
                    let mut builder = self.schedule.clone();

                    builder.reorganize_schedule(|machines, _| {
                        let task = machines[self.machine].remove(self.i);
                        machines[self.machine].insert(self.j, task);
                        (vec![(self.machine, self.i.min(self.j))], vec![])
                    });

                    self.j += 1;

                    return Some(builder);
                }
                self.i += 1;
            }
            self.machine += 1;
        }
        None
    }
}

/// Neighborhood that swaps tasks on different machines.
pub struct SwapTwoMachines<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    first: usize,
    second: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of SwapTwoMachines neighborhood.
pub fn swap_two_machines<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(SwapTwoMachines {
        schedule,
        first: 0,
        second: 1,
        i: 0,
        j: 0,
    })
}

impl<'a, 'b> Iterator for SwapTwoMachines<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.first + 1 < self.schedule.machines_len() {
            while self.second < self.schedule.machines_len() {
                while self.i < self.schedule.machine_tasks_len(self.first) {
                    if self.j < self.schedule.machine_tasks_len(self.second) {
                        let mut builder = self.schedule.clone();

                        builder.reorganize_schedule(|machines, _| {
                            let value = machines[self.first][self.i];
                            machines[self.first][self.i] = machines[self.second][self.j];
                            machines[self.second][self.j] = value;

                            (vec![(self.first, self.i), (self.second, self.j)], vec![])
                        });

                        self.j += 1;

                        return Some(builder);
                    }
                    self.i += 1;
                }
                self.second += 1;
            }
            self.first += 1;
        }
        None
    }
}

/// Neighborhood that moves task on different machine.
pub struct MoveTwoMachines<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    first: usize,
    second: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of MoveTwoMachines neighborhood.
pub fn move_two_machines<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(MoveTwoMachines {
        schedule,
        first: 0,
        second: 1,
        i: 0,
        j: 0,
    })
}

impl<'a, 'b> Iterator for MoveTwoMachines<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.first + 1 < self.schedule.machines_len() {
            while self.second < self.schedule.machines_len() {
                while self.i < self.schedule.machine_tasks_len(self.first) {
                    if self.j <= self.schedule.machine_tasks_len(self.second) {
                        let mut builder = self.schedule.clone();

                        builder.reorganize_schedule(|machines, _| {
                            let value = machines[self.first].remove(self.i);
                            machines[self.second].insert(self.j, value);

                            (vec![(self.first, self.i), (self.second, self.j)], vec![])
                        });

                        self.j += 1;

                        return Some(builder);
                    }
                    self.i += 1;
                }
                self.second += 1;
            }
            self.first += 1;
        }
        None
    }
}

/// Neighborhood that replaces task with a tardy task.
pub struct ReplaceWithTardy<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of ReplaceWithTardy neighborhood.
pub fn replace_with_tardy<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(ReplaceWithTardy {
        schedule,
        machine: 0,
        i: 0,
        j: 0,
    })
}

impl<'a, 'b> Iterator for ReplaceWithTardy<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.machine < self.schedule.machines_len() {
            while self.i < self.schedule.machine_tasks_len(self.machine) {
                if self.j < self.schedule.tardy_len() {
                    let mut builder = self.schedule.clone();

                    builder.reorganize_schedule(|machines, tardy_tasks| {
                        std::mem::swap(
                            &mut machines[self.machine][self.i],
                            &mut tardy_tasks[self.j],
                        );

                        (vec![(self.machine, self.i)], vec![tardy_tasks[self.j]])
                    });

                    self.j += 1;

                    return Some(builder);
                }
                self.i += 1;
            }
            self.machine += 1;
        }
        None
    }
}

/// Neighborhood that adds a tardy task.
pub struct AddTardy<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

/// Creates a new instance of AddTardy neighborhood.
pub fn add_tardy<'a, 'b>(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
    Box::new(AddTardy {
        schedule,
        machine: 0,
        i: 0,
        j: 0,
    })
}

impl<'a, 'b> Iterator for AddTardy<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.machine < self.schedule.machines_len() {
            while self.i <= self.schedule.machine_tasks_len(self.machine) {
                if self.j < self.schedule.tardy_len() {
                    let mut builder = self.schedule.clone();

                    builder.reorganize_schedule(|machines, tardy_tasks| {
                        machines[self.machine].insert(self.i, tardy_tasks[self.j]);
                        tardy_tasks.remove(self.j);

                        (vec![(self.machine, self.i)], vec![])
                    });

                    self.j += 1;

                    return Some(builder);
                }
                self.i += 1;
            }
            self.machine += 1;
        }
        None
    }
}
