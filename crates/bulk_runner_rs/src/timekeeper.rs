use crate::info;
use chrono::Local;

struct Time {
    start: tokio::time::Instant,
    elapsed: tokio::time::Duration,
}

pub struct TimeKeeper {
    datetime: chrono::DateTime<Local>,
    time: Time,
}

impl TimeKeeper {
    pub fn new() -> Self {
        let datetime = Local::now();
        let time = Time {
            start: tokio::time::Instant::now(),
            elapsed: tokio::time::Duration::default(),
        };

        TimeKeeper { datetime, time }
    }

    pub fn elapsed(&self) -> tokio::time::Duration {
        self.time.elapsed
    }

    pub fn datetime(&self) -> chrono::DateTime<Local> {
        self.datetime
    }

    pub fn print_elapsed(&self) {
        info!(
            "->> {:<12} - {:?}",
            "TIME:: Elapsed time",
            self.time.start.elapsed()
        );
    }

    // Need to format this better cos it's insanely difficult to read as it's in tz style
    pub fn print_started_at(&self) {
        // let dt = self.datetime();
        let dt = self
            .datetime()
            .fixed_offset()
            .format("%Y-%m-%d || %H:%M:%S")
            .to_string();
        // dt.format("%Y-%m-%d || %H:%M:%S").to_string();
        info!("->> {:<12} - {:?}", "TIME:: Started at", dt);
    }
}
