enum LogLevel {
  Debug,
  Info,
  Warning,
  Error
}

pub struct LogMessage {
  level: LogLevel,
  message: String
}
