#![no_std]
#![forbid(unsafe_code)]

pub const MAX_PERIODIC_TASKS: usize = 32;
pub const MAX_ONE_OFF_TASKS: usize = 64;

use core::time::Duration;

#[derive(Debug, Clone, Copy)]
struct TaskData {
    task: fn(),
    start: u32,
    duration: u32,
}

pub struct TaskHandler {
    periodic_tasks: [Option<TaskData>; MAX_PERIODIC_TASKS],
    one_off_tasks: [Option<TaskData>; MAX_ONE_OFF_TASKS],
    one_off_index: usize,
    now: fn() -> u32,
    duration_to_count: fn(Duration) -> u32,
}

impl TaskHandler {
    pub fn new(now: fn() -> u32, duration_to_count: fn(Duration) -> u32) -> Self {
        Self {
            periodic_tasks: [None; MAX_PERIODIC_TASKS],
            one_off_tasks: [None; MAX_ONE_OFF_TASKS],
            one_off_index: 0,
            now,
            duration_to_count,
        }
    }

    pub fn schedule_periodic(&mut self, task: fn(), period: Duration) {
        for periodic_task in self.periodic_tasks.iter_mut() {
            if periodic_task.is_none() {
                *periodic_task = Some(TaskData {
                    task,
                    start: (self.now)(),
                    duration: (self.duration_to_count)(period),
                });
            }
        }
    }

    pub fn schedule_periodic_delayed(&mut self, task: fn(), period: Duration, delay: Duration) {
        for periodic_task in self.periodic_tasks.iter_mut() {
            if periodic_task.is_none() {
                *periodic_task = Some(TaskData {
                    task,
                    start: (self.now)() + (self.duration_to_count)(delay),
                    duration: (self.duration_to_count)(period),
                });
            }
        }
    }

    pub fn schedule_one_off(&mut self, task: fn(), wait: Duration) {
        if self.one_off_tasks[self.one_off_index].is_none() {
            self.one_off_tasks[self.one_off_index] = Some(TaskData {
                task,
                start: (self.now)(),
                duration: (self.duration_to_count)(wait),
            });
        } else {
            for one_off_task in self.one_off_tasks.iter_mut() {
                if one_off_task.is_none() {
                    *one_off_task = Some(TaskData {
                        task,
                        start: (self.now)(),
                        duration: (self.duration_to_count)(wait),
                    });
                }
            }
        }
    }

    pub fn update(&mut self) {
        let mut now = (self.now)();

        for periodic_task in self.periodic_tasks.iter_mut() {
            if let Some(task) = periodic_task {
                if now.wrapping_sub(task.start) >= task.duration {
                    task.start = now;
                    (task.task)();
                    now = (self.now)();
                }
            }
        }

        for (i, one_off_task) in self.one_off_tasks.iter_mut().enumerate() {
            if let Some(task) = one_off_task {
                if now.wrapping_sub(task.start) >= task.duration {
                    (task.task)();
                    *one_off_task = None;
                    self.one_off_index = i;
                    now = (self.now)();
                }
            }
        }
    }
}

