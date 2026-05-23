pub mod aggregator;
pub use aggregator::AnalysisResult;

use anyhow::Result;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use crate::filter::Filter;
use crate::{LogEntry, LogLevel};
use crate::parser;

pub struct LogAnalyzer {
    entries: Vec<LogEntry>,
    filter: Filter,
}

impl LogAnalyzer {
    pub fn load_files<P: AsRef<Path> + Sync>(paths: &[P]) -> Result<Self> {
        let entries: Vec<LogEntry> = paths
            .par_iter()
            .flat_map(|path| {
                let content = match fs::read_to_string(path.as_ref()) {
                    Ok(c) => c,
                    Err(_) => return Vec::new(),
                };
                content
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .filter_map(|line| {
                        let line = line.trim_end_matches('\r');
                        let p = parser::auto_detect(line);
                        p.parse(line)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(LogAnalyzer {
            entries,
            filter: Filter::default(),
        })
    }

    pub fn load_stdin() -> Result<Self> {
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let entries: Vec<LogEntry> = stdin
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| !l.trim().is_empty())
            .filter_map(|line| {
                let line = line.trim_end_matches('\r').to_string();
                let p = parser::auto_detect(&line);
                p.parse(&line)
            })
            .collect();

        Ok(LogAnalyzer {
            entries,
            filter: Filter::default(),
        })
    }

    pub fn filter_by_level(mut self, level: LogLevel) -> Self {
        self.filter.level = Some(level);
        self
    }

    pub fn filter_by_source(mut self, source: impl Into<String>) -> Self {
        self.filter.source = Some(source.into());
        self
    }

    pub fn filter_by_message(mut self, pattern: impl Into<String>) -> Self {
        self.filter.message_contains = Some(pattern.into());
        self
    }

    pub fn filter_by_time_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.filter.from = Some(from);
        self.filter.to = Some(to);
        self
    }

    pub fn analyze(self) -> AnalysisResult {
        let filter = self.filter;
        let filtered: Vec<LogEntry> = self
            .entries
            .into_iter()
            .filter(|e| filter.matches(e))
            .collect();

        aggregator::aggregate(filtered)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
