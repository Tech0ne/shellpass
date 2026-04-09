use std::time::{Duration, Instant};

const NOTIFICATION_DURATION: Duration = Duration::from_secs(3);

pub struct Notification {
    pub message: String,
    pub created: Instant,
    pub error: bool,
}

impl Notification {
    pub fn info(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            created: Instant::now(),
            error: false,
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
            created: Instant::now(),
            error: true,
        }
    }

    pub fn expired(&self) -> bool {
        self.created.elapsed() > NOTIFICATION_DURATION
    }
}
