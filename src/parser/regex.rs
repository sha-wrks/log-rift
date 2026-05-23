use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use regex::Regex;
use std::str::FromStr;

use crate::{LogEntry, LogLevel};
use super::LogParser;

pub struct RegexLogParser {
    // 2024-01-15T12:00:00Z ERROR [source] message
    // 2024-01-15 12:00:00.000 ERROR source: message
    iso_re: Regex,
    // [2024-01-15 12:00:00] [ERROR] message
    bracketed_re: Regex,
    // Jan 15 12:00:00 host process[pid]: message
    syslog_re: Regex,
    // ERROR: message  or  ERROR message
    simple_re: Regex,
}

impl Default for RegexLogParser {
    fn default() -> Self {
        Self {
            iso_re: Regex::new(
                r"(?i)^(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?(?:Z|[+-]\d{2}:?\d{2})?)\s+(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|ERR|FATAL|CRITICAL)\s*(?:\[([^\]]+)\])?\s*(.*)$"
            ).unwrap(),
            bracketed_re: Regex::new(
                r"(?i)^\[(\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}[^\]]*)\]\s+\[?(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|FATAL)\]?\s+(.*)$"
            ).unwrap(),
            syslog_re: Regex::new(
                r"^(\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+\S+\s+([^\[:\s]+)(?:\[\d+\])?:\s+(.+)$"
            ).unwrap(),
            simple_re: Regex::new(
                r"(?i)^(TRACE|DEBUG|INFO|WARN(?:ING)?|ERROR|ERR|FATAL|CRITICAL)\s*[:\-]?\s+(.+)$"
            ).unwrap(),
        }
    }
}

impl LogParser for RegexLogParser {
    fn parse(&self, line: &str) -> Option<LogEntry> {
        let line = line.trim();

        if let Some(caps) = self.iso_re.captures(line) {
            let ts_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let level_str = caps.get(2).map(|m| m.as_str()).unwrap_or("INFO");
            let source = caps.get(3).map(|m| m.as_str()).unwrap_or("unknown").to_string();
            let message = caps.get(4).map(|m| m.as_str()).unwrap_or("").to_string();

            return Some(LogEntry {
                timestamp: parse_timestamp(ts_str),
                level: LogLevel::from_str(level_str).unwrap_or(LogLevel::Info),
                source,
                message,
                fields: Default::default(),
            });
        }

        if let Some(caps) = self.bracketed_re.captures(line) {
            let ts_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let level_str = caps.get(2).map(|m| m.as_str()).unwrap_or("INFO");
            let message = caps.get(3).map(|m| m.as_str()).unwrap_or("").to_string();

            return Some(LogEntry {
                timestamp: parse_timestamp(ts_str),
                level: LogLevel::from_str(level_str).unwrap_or(LogLevel::Info),
                source: "unknown".to_string(),
                message,
                fields: Default::default(),
            });
        }

        if let Some(caps) = self.syslog_re.captures(line) {
            let source = caps.get(2).map(|m| m.as_str()).unwrap_or("unknown").to_string();
            let message = caps.get(3).map(|m| m.as_str()).unwrap_or("").to_string();

            return Some(LogEntry {
                timestamp: None,
                level: LogLevel::Info,
                source,
                message,
                fields: Default::default(),
            });
        }

        if let Some(caps) = self.simple_re.captures(line) {
            let level_str = caps.get(1).map(|m| m.as_str()).unwrap_or("INFO");
            let message = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();

            return Some(LogEntry {
                timestamp: None,
                level: LogLevel::from_str(level_str).unwrap_or(LogLevel::Info),
                source: "unknown".to_string(),
                message,
                fields: Default::default(),
            });
        }

        // Fallback: treat entire line as an INFO message
        if !line.is_empty() {
            return Some(LogEntry {
                timestamp: None,
                level: LogLevel::Info,
                source: "unknown".to_string(),
                message: line.to_string(),
                fields: Default::default(),
            });
        }

        None
    }
}

fn parse_timestamp(s: &str) -> Option<DateTime<Utc>> {
    // Try RFC3339 / ISO 8601 with timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    // Try without timezone suffix (assume UTC)
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
    ];
    for fmt in &formats {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(Utc.from_utc_datetime(&ndt));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_iso_format() {
        let parser = RegexLogParser::default();
        let line = "2024-01-15T12:00:00Z ERROR [myapp] Connection timeout";

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.source, "myapp");
        assert_eq!(entry.message, "Connection timeout");
        assert!(entry.timestamp.is_some());
    }

    #[test]
    fn test_parse_space_separated_timestamp() {
        let parser = RegexLogParser::default();
        let line = "2024-01-15 12:00:00.000 WARN Service degraded";

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Warn);
    }

    #[test]
    fn test_parse_simple_level_prefix() {
        let parser = RegexLogParser::default();
        let line = "ERROR: disk full";

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "disk full");
    }

    #[test]
    fn test_parse_bracketed_format() {
        let parser = RegexLogParser::default();
        let line = "[2024-01-15 12:00:00] [FATAL] system crash";

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Fatal);
        assert_eq!(entry.message, "system crash");
    }
}
