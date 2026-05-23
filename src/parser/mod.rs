pub mod json;
pub mod regex;
pub mod structured;

pub use json::JsonLogParser;
pub use regex::RegexLogParser;
pub use structured::StructuredLogParser;

use crate::LogEntry;

pub trait LogParser: Send + Sync {
    fn parse(&self, line: &str) -> Option<LogEntry>;
}

pub fn auto_detect(line: &str) -> Box<dyn LogParser> {
    let trimmed = line.trim();
    if trimmed.starts_with('{') {
        Box::new(JsonLogParser)
    } else if trimmed.contains('=') && !trimmed.contains('[') && !trimmed.starts_with('#') {
        Box::new(StructuredLogParser)
    } else {
        Box::new(RegexLogParser::default())
    }
}
