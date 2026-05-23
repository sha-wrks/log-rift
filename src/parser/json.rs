use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::{LogEntry, LogLevel};
use super::LogParser;

pub struct JsonLogParser;

impl LogParser for JsonLogParser {
    fn parse(&self, line: &str) -> Option<LogEntry> {
        let json: serde_json::Value = serde_json::from_str(line).ok()?;

        Some(LogEntry {
            timestamp: json
                .get("timestamp")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            level: json
                .get("level")
                .and_then(|v| v.as_str())
                .and_then(|s| LogLevel::from_str(s).ok())
                .unwrap_or(LogLevel::Info),
            source: json
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            message: json
                .get("message")
                .or_else(|| json.get("msg"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            fields: json
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .filter(|(k, _)| {
                            !["timestamp", "level", "source", "message", "msg"]
                                .contains(&k.as_str())
                        })
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_json_log() {
        let parser = JsonLogParser;
        let line = r#"{"timestamp":"2024-01-15T12:00:00Z","level":"ERROR","source":"app","message":"Connection failed"}"#;

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Connection failed");
        assert_eq!(entry.source, "app");
    }

    #[test]
    fn test_parse_invalid_json() {
        let parser = JsonLogParser;
        assert!(parser.parse("not json").is_none());
    }

    #[test]
    fn test_parse_missing_fields() {
        let parser = JsonLogParser;
        let line = r#"{"level":"WARN"}"#;

        let entry = parser.parse(line).expect("Should parse with defaults");
        assert_eq!(entry.message, "");
        assert_eq!(entry.source, "unknown");
        assert_eq!(entry.level, LogLevel::Warn);
    }

    #[test]
    fn test_parse_msg_alias() {
        let parser = JsonLogParser;
        let line = r#"{"level":"INFO","msg":"hello world"}"#;

        let entry = parser.parse(line).expect("Should parse msg alias");
        assert_eq!(entry.message, "hello world");
    }

    #[test]
    fn test_extra_fields_captured() {
        let parser = JsonLogParser;
        let line = r#"{"level":"INFO","message":"test","request_id":"abc123","latency_ms":42}"#;

        let entry = parser.parse(line).expect("Should parse");
        assert!(entry.fields.contains_key("request_id"));
        assert!(entry.fields.contains_key("latency_ms"));
    }
}
