pub trait Logger {
    fn log(&mut self, level: LogLevel, msg: String);
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    Info
}

pub struct ConsoleLogger {
    name: String
}

impl Logger for ConsoleLogger {
    fn log(&mut self, level: LogLevel, msg: String) {
        println!("[{}] ({:?}): {}", &self.name, level, msg);
    }
}

impl ConsoleLogger {
    pub fn new(name: String) -> Self {
        ConsoleLogger{ name: name }
    }
}
