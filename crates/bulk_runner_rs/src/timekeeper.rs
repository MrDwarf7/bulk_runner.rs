use chrono::Local;

use crate::info;

struct Time {
    start:   tokio::time::Instant,
    elapsed: tokio::time::Duration,
}

pub struct TimeKeeper {
    datetime: chrono::DateTime<Local>,
    time:     Time,
}

impl TimeKeeper {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        let datetime = Local::now();
        let time = Time {
            start:   tokio::time::Instant::now(),
            elapsed: tokio::time::Duration::default(),
        };

        TimeKeeper { datetime, time }
    }

    #[must_use]
    #[inline]
    pub fn elapsed(&self) -> tokio::time::Duration {
        self.time.elapsed
    }

    #[must_use]
    #[inline]
    pub fn datetime(&self) -> chrono::DateTime<Local> {
        self.datetime
    }

    #[inline]
    pub fn print_elapsed(&self) {
        info!("->> {:<12} - {:?}", "TIME:: Elapsed time", self.time.start.elapsed());
    }

    // Need to format this better cos it's insanely difficult to read as it's in tz style
    #[inline]
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

impl Default for TimeKeeper {
    #[inline]
    fn default() -> Self {
        let datetime = Local::now();
        let time = Time {
            start:   tokio::time::Instant::now(),
            elapsed: tokio::time::Duration::default(),
        };

        TimeKeeper { datetime, time }
    }
}
