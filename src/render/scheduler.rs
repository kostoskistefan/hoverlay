use std::time::{Duration, Instant};

const RENDER_INTERVAL: Duration = Duration::from_secs(1);

pub struct Scheduler {
    mode: ScheduleMode,
}

enum ScheduleMode {
    Static,
    Periodic {
        interval: Duration,
        next_render_time: Instant,
    },
}

pub enum Schedule {
    Wait,
    WaitUntil(Instant),
    Render,
}

impl Scheduler {
    pub fn new(is_static: bool) -> Self {
        let mode = if is_static {
            ScheduleMode::Static
        } else {
            ScheduleMode::Periodic {
                interval: RENDER_INTERVAL,
                next_render_time: Instant::now() + RENDER_INTERVAL,
            }
        };

        Self { mode }
    }

    pub fn next(&mut self) -> Schedule {
        match &mut self.mode {
            ScheduleMode::Static => Schedule::Wait,

            ScheduleMode::Periodic {
                interval,
                next_render_time,
            } => {
                let now = Instant::now();

                if now < *next_render_time {
                    Schedule::WaitUntil(*next_render_time)
                } else {
                    *next_render_time += *interval;
                    Schedule::Render
                }
            }
        }
    }
}
