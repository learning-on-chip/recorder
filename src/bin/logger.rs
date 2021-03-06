use log::{self, Log, LogLevel, LogLevelFilter, LogMetadata, LogRecord};

pub struct Logger;

impl Logger {
    #[allow(unused_must_use)]
    pub fn install() {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(Logger)
        });
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("[{:10}] {}", record.target(), record.args());
        }
    }
}
