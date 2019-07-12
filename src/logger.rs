pub trait Logger {
    fn log(&mut self, level: LogLevel, msg: String);

    fn info(&mut self, msg: String) {
        self.log(LogLevel::Info, msg);
    }

    fn debug(&mut self, msg: String) {
        self.log(LogLevel::Debug, msg);
    }

    fn error(&mut self, msg: String) {
        self.log(LogLevel::Error, msg);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Info,
    Debug,
    Error,
}

pub struct ConsoleLogger {
    name: String,
}

impl Logger for ConsoleLogger {
    fn log(&mut self, level: LogLevel, msg: String) {
        println!("[{}] ({:?}): {}", &self.name, level, msg);
    }
}

impl ConsoleLogger {
    pub fn new(name: String) -> Self {
        ConsoleLogger { name: name }
    }
}
