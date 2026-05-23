pub mod analyzer;
pub mod filter;
pub mod output;
pub mod parser;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

pub use analyzer::LogAnalyzer;
pub use parser::LogParser;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl FromStr for LogLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" | "TRC" => Ok(LogLevel::Trace),
            "DEBUG" | "DBG" => Ok(LogLevel::Debug),
            "INFO" | "INF" => Ok(LogLevel::Info),
            "WARN" | "WARNING" | "WRN" => Ok(LogLevel::Warn),
            "ERROR" | "ERR" => Ok(LogLevel::Error),
            "FATAL" | "CRITICAL" | "CRIT" => Ok(LogLevel::Fatal),
            _ => Err(anyhow::anyhow!("Unknown log level: {}", s)),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub fields: HashMap<String, String>,
}
