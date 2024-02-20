use crate::util::ScheduleBuilder;

/// Neighborhood trait represents a neighborhood of a schedule.
/// It's used to generate new schedules from a given schedule.
pub type Neighborhood<'a, 'b> = dyn Iterator<Item = ScheduleBuilder<'a>> + 'b;

/// Factory for creating a neighborhood.
pub type NeighborhoodFactory<'a, 'b> = fn(&'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>>;

/// Returns all neighborhoods factories.
pub fn factories<'a, 'b>() -> Box<[NeighborhoodFactory<'a, 'b>]> {
    Box::new([
        SwapSingleMachine::new_boxed,
        MoveSingleMachine::new_boxed,
        SwapTwoMachines::new_boxed,
        MoveTwoMachines::new_boxed,
        ReplaceWithTardy::new_boxed,
        AddTardy::new_boxed,
    ])
}

/// Neighborhood that swaps two tasks on the same machine.
pub struct SwapSingleMachine<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

impl<'a, 'b> SwapSingleMachine<'a, 'b> {
    /// Creates a new swap single machine neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            machine: 0,
            i: 0,
            j: 1,
        })
    }
}

impl<'a, 'b> Iterator for SwapSingleMachine<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.schedule.machines[self.machine].len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        while self.machine < self.schedule.machines.len()
            && self.i >= self.schedule.machines[self.machine].len() - 1
        {
            self.machine += 1;
            self.i = 0;
            self.j = 1;
        }

        if self.machine >= self.schedule.machines.len() {
            return None;
        }

        let mut builder = self.schedule.clone();

        builder.machines[self.machine].swap(self.i, self.j);
        for &task in &builder.machines[self.machine][self.i..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.machine, self.i);
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
    }
}

/// Neighborhood that moves task on the same machine.
pub struct MoveSingleMachine<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

impl<'a, 'b> MoveSingleMachine<'a, 'b> {
    /// Creates a new move single task on a single machine neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            machine: 0,
            i: 0,
            j: 1,
        })
    }
}

impl<'a, 'b> Iterator for MoveSingleMachine<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.schedule.machines[self.machine].len() {
            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.schedule.machines[self.machine].len() {
            self.machine += 1;
            self.i = 0;
        }

        if self.machine >= self.schedule.machines.len() {
            return None;
        }

        let mut builder = self.schedule.clone();

        let task = builder.machines[self.machine].remove(self.i);
        builder.machines[self.machine].insert(self.j, task);
        for &task in &builder.machines[self.machine][self.i.min(self.j)..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.machine, self.i.min(self.j));
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
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

impl<'a, 'b> SwapTwoMachines<'a, 'b> {
    /// Creates a new swap two machine neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            first: 0,
            second: 1,
            i: 0,
            j: 0,
        })
    }
}

impl<'a, 'b> Iterator for SwapTwoMachines<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.schedule.machines[self.second].len() {
            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.schedule.machines[self.first].len() {
            self.second += 1;
            self.i = 0;
        }

        if self.second >= self.schedule.machines.len() {
            self.first += 1;
            self.second = self.first + 1;
        }

        if self.first >= self.schedule.machines.len() - 1 {
            return None;
        }

        let mut builder = self.schedule.clone();

        let value = builder.machines[self.first][self.i];
        builder.machines[self.first][self.i] = builder.machines[self.second][self.j];
        builder.machines[self.second][self.j] = value;

        for &task in &builder.machines[self.first][self.i..] {
            builder.schedule.remove_schedule(task);
        }

        for &task in &builder.machines[self.second][self.j..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.first, self.i);
        builder.fix_machine(self.second, self.j);
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
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

impl<'a, 'b> MoveTwoMachines<'a, 'b> {
    /// Creates a new move two machine neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            first: 0,
            second: 1,
            i: 0,
            j: 0,
        })
    }
}

impl<'a, 'b> Iterator for MoveTwoMachines<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j > self.schedule.machines[self.second].len() {
            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.schedule.machines[self.first].len() {
            self.second += 1;
            self.i = 0;
        }

        if self.second >= self.schedule.machines.len() {
            self.first += 1;
            self.second = self.first + 1;
        }

        if self.first >= self.schedule.machines.len() - 1 {
            return None;
        }

        let mut builder = self.schedule.clone();

        let value = builder.machines[self.first].remove(self.i);
        builder.machines[self.second].insert(self.j, value);

        for &task in &builder.machines[self.first][self.i..] {
            builder.schedule.remove_schedule(task);
        }

        for &task in &builder.machines[self.second][self.j..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.first, self.i);
        builder.fix_machine(self.second, self.j);
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
    }
}

/// Neighborhood that replaces task with a tardy task.
pub struct ReplaceWithTardy<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

impl<'a, 'b> ReplaceWithTardy<'a, 'b> {
    /// Creates a new replacement tardy neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            machine: 0,
            i: 0,
            j: 0,
        })
    }
}

impl<'a, 'b> Iterator for ReplaceWithTardy<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.schedule.tardy_tasks.len() {
            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.schedule.machines[self.machine].len() {
            self.machine += 1;
            self.i = 0;
        }

        if self.machine >= self.schedule.machines.len() || self.schedule.tardy_tasks.is_empty() {
            return None;
        }

        let mut builder = self.schedule.clone();

        std::mem::swap(
            &mut builder.machines[self.machine][self.i],
            &mut builder.tardy_tasks[self.j],
        );

        builder
            .schedule
            .remove_schedule(builder.tardy_tasks[self.j]);
        for &task in &builder.machines[self.machine][self.i..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.machine, self.i);
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
    }
}

/// Neighborhood that adds a tardy task.
pub struct AddTardy<'a, 'b> {
    schedule: &'b ScheduleBuilder<'a>,
    machine: usize,
    i: usize,
    j: usize,
}

impl<'a, 'b> AddTardy<'a, 'b> {
    /// Creates add a tardy neighbourhood.
    pub fn new_boxed(schedule: &'b ScheduleBuilder<'a>) -> Box<Neighborhood<'a, 'b>> {
        Box::new(Self {
            schedule,
            machine: 0,
            i: 0,
            j: 0,
        })
    }
}

impl<'a, 'b> Iterator for AddTardy<'a, 'b> {
    type Item = ScheduleBuilder<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.schedule.tardy_tasks.len() {
            self.i += 1;
            self.j = 0;
        }

        if self.i > self.schedule.machines[self.machine].len() {
            self.machine += 1;
            self.i = 0;
        }

        if self.machine >= self.schedule.machines.len() || self.schedule.tardy_tasks.is_empty() {
            return None;
        }

        let mut builder = self.schedule.clone();

        builder.machines[self.machine].insert(self.i, builder.tardy_tasks[self.j]);
        builder.tardy_tasks.remove(self.j);

        for &task in &builder.machines[self.machine][self.i..] {
            builder.schedule.remove_schedule(task);
        }

        builder.fix_machine(self.machine, self.i);
        builder.fix_tardy();

        self.j += 1;

        Some(builder)
    }
}
