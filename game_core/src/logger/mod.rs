use log::{Level, Log, Metadata, Record};

struct Logger {
    log_level: Level,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            log_level: Level::Debug,
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        //        self.log_level = record.level();
    }

    fn flush(&self) {}
}
