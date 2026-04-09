use std::time::{Duration, Instant};

pub struct ClipTimer {
    pub start: Instant,
    pub duration: Duration,
    pub label: String,
}

impl ClipTimer {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            duration: Duration::from_secs(10),
            label: label.into(),
        }
    }

    pub fn remaining_frac(&self) -> f64 {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            0.0
        } else {
            1.0 - elapsed.as_secs_f64() / self.duration.as_secs_f64()
        }
    }

    pub fn remaining_secs(&self) -> u64 {
        let elapsed = self.start.elapsed();
        if elapsed >= self.duration {
            0
        } else {
            (self.duration - elapsed).as_secs()
        }
    }

    pub fn expired(&self) -> bool {
        self.start.elapsed() >= self.duration
    }
}
