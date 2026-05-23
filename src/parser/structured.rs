use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::str::FromStr;

use crate::{LogEntry, LogLevel};
use super::LogParser;

/// Parses logfmt / key=value structured log lines.
/// Example: ts=2024-01-15T12:00:00Z level=error source=app msg="Connection failed" latency=100ms
pub struct StructuredLogParser;

impl LogParser for StructuredLogParser {
    fn parse(&self, line: &str) -> Option<LogEntry> {
        let pairs = parse_kv(line.trim());
        if pairs.is_empty() {
            return None;
        }

        let get = |key: &str| pairs.get(key).map(|s| s.as_str());

        let timestamp = get("ts")
            .or_else(|| get("timestamp"))
            .or_else(|| get("time"))
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let level = get("level")
            .or_else(|| get("lvl"))
            .or_else(|| get("severity"))
            .and_then(|s| LogLevel::from_str(s).ok())
            .unwrap_or(LogLevel::Info);

        let source = get("source")
            .or_else(|| get("service"))
            .or_else(|| get("app"))
            .or_else(|| get("logger"))
            .unwrap_or("unknown")
            .to_string();

        let message = get("msg")
            .or_else(|| get("message"))
            .or_else(|| get("text"))
            .unwrap_or("")
            .to_string();

        let reserved = ["ts", "timestamp", "time", "level", "lvl", "severity",
                         "source", "service", "app", "logger", "msg", "message", "text"];

        let fields: HashMap<String, String> = pairs
            .into_iter()
            .filter(|(k, _)| !reserved.contains(&k.as_str()))
            .collect();

        Some(LogEntry { timestamp, level, source, message, fields })
    }
}

fn parse_kv(line: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let b = line.as_bytes();
    let len = b.len();
    let mut i = 0;

    while i < len {
        // Skip whitespace
        while i < len && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }

        // Read key (up to '=' or whitespace)
        let key_start = i;
        while i < len && b[i] != b'=' && !b[i].is_ascii_whitespace() {
            i += 1;
        }
        let key = &line[key_start..i];
        if key.is_empty() || i >= len || b[i] != b'=' {
            // Token without '=', skip it
            while i < len && !b[i].is_ascii_whitespace() {
                i += 1;
            }
            continue;
        }
        i += 1; // consume '='

        // Read value (quoted or unquoted)
        let value = if i < len && b[i] == b'"' {
            i += 1; // consume opening '"'
            let mut val = String::new();
            while i < len {
                if b[i] == b'\\' && i + 1 < len {
                    i += 1;
                    val.push(b[i] as char);
                    i += 1;
                } else if b[i] == b'"' {
                    i += 1; // consume closing '"'
                    break;
                } else {
                    val.push(b[i] as char);
                    i += 1;
                }
            }
            val
        } else {
            let val_start = i;
            while i < len && !b[i].is_ascii_whitespace() {
                i += 1;
            }
            line[val_start..i].to_string()
        };

        map.insert(key.to_string(), value);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_kv() {
        let parser = StructuredLogParser;
        let line = r#"ts=2024-01-15T12:00:00Z level=error source=app msg="Connection failed""#;

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.source, "app");
        assert_eq!(entry.message, "Connection failed");
        assert!(entry.timestamp.is_some());
    }

    #[test]
    fn test_parse_logfmt_aliases() {
        let parser = StructuredLogParser;
        let line = r#"time=2024-01-15T12:00:00Z lvl=warn service=api text="high latency""#;

        let entry = parser.parse(line).expect("Should parse");
        assert_eq!(entry.level, LogLevel::Warn);
        assert_eq!(entry.source, "api");
        assert_eq!(entry.message, "high latency");
    }

    #[test]
    fn test_extra_fields_in_fields_map() {
        let parser = StructuredLogParser;
        let line = r#"level=info msg=ok request_id=abc123 duration=42ms"#;

        let entry = parser.parse(line).expect("Should parse");
        assert!(entry.fields.contains_key("request_id"));
        assert!(entry.fields.contains_key("duration"));
    }

    #[test]
    fn test_parse_empty_line() {
        let parser = StructuredLogParser;
        assert!(parser.parse("").is_none());
        assert!(parser.parse("   ").is_none());
    }
}
