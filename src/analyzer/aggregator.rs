use std::collections::HashMap;

use crate::{LogEntry, LogLevel};

#[derive(Debug, serde::Serialize)]
pub struct AnalysisResult {
    pub count: usize,
    pub entries: Vec<LogEntry>,
    pub level_counts: HashMap<String, usize>,
    pub source_counts: HashMap<String, usize>,
    pub error_rate: f64,
}

pub fn aggregate(entries: Vec<LogEntry>) -> AnalysisResult {
    let count = entries.len();
    let mut level_counts: HashMap<String, usize> = HashMap::new();
    let mut source_counts: HashMap<String, usize> = HashMap::new();
    let mut error_count: usize = 0;

    for entry in &entries {
        *level_counts.entry(entry.level.to_string()).or_insert(0) += 1;
        *source_counts.entry(entry.source.clone()).or_insert(0) += 1;
        if matches!(entry.level, LogLevel::Error | LogLevel::Fatal) {
            error_count += 1;
        }
    }

    let error_rate = if count > 0 {
        error_count as f64 / count as f64 * 100.0
    } else {
        0.0
    };

    AnalysisResult {
        count,
        entries,
        level_counts,
        source_counts,
        error_rate,
    }
}
