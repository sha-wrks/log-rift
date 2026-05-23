use chrono::{DateTime, Utc};

use crate::{LogEntry, LogLevel};

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub level: Option<LogLevel>,
    pub source: Option<String>,
    pub message_contains: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn matches(&self, entry: &LogEntry) -> bool {
        if let Some(ref min_level) = self.level {
            if &entry.level < min_level {
                return false;
            }
        }

        if let Some(ref src) = self.source {
            if !entry.source.to_lowercase().contains(&src.to_lowercase()) {
                return false;
            }
        }

        if let Some(ref pattern) = self.message_contains {
            if !entry.message.to_lowercase().contains(&pattern.to_lowercase()) {
                return false;
            }
        }

        if let Some(from) = self.from {
            match entry.timestamp {
                Some(ts) if ts >= from => {}
                _ => return false,
            }
        }

        if let Some(to) = self.to {
            match entry.timestamp {
                Some(ts) if ts <= to => {}
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogLevel;
    use std::collections::HashMap;

    fn make_entry(level: LogLevel, source: &str, message: &str) -> LogEntry {
        LogEntry {
            timestamp: None,
            level,
            source: source.to_string(),
            message: message.to_string(),
            fields: HashMap::new(),
        }
    }

    #[test]
    fn test_level_filter() {
        let mut f = Filter::new();
        f.level = Some(LogLevel::Error);

        assert!(!f.matches(&make_entry(LogLevel::Info, "app", "msg")));
        assert!(!f.matches(&make_entry(LogLevel::Warn, "app", "msg")));
        assert!(f.matches(&make_entry(LogLevel::Error, "app", "msg")));
        assert!(f.matches(&make_entry(LogLevel::Fatal, "app", "msg")));
    }

    #[test]
    fn test_source_filter() {
        let mut f = Filter::new();
        f.source = Some("database".to_string());

        assert!(f.matches(&make_entry(LogLevel::Info, "database", "msg")));
        assert!(f.matches(&make_entry(LogLevel::Info, "my-database-pool", "msg")));
        assert!(!f.matches(&make_entry(LogLevel::Info, "webserver", "msg")));
    }

    #[test]
    fn test_message_filter() {
        let mut f = Filter::new();
        f.message_contains = Some("timeout".to_string());

        assert!(f.matches(&make_entry(LogLevel::Error, "app", "Connection Timeout exceeded")));
        assert!(!f.matches(&make_entry(LogLevel::Error, "app", "Connection refused")));
    }
}
