use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

use logagg::analyzer::LogAnalyzer;
use logagg::output::{self, OutputFormat};
use logagg::LogLevel;

#[derive(Parser)]
#[command(
    name = "logagg",
    about = "A fast, flexible command-line log analyzer",
    version
)]
struct Cli {
    /// Log files to analyze (reads from stdin if omitted)
    files: Vec<PathBuf>,

    /// Filter by minimum log level
    #[arg(short, long, value_name = "LEVEL")]
    level: Option<String>,

    /// Filter by source (substring match)
    #[arg(short, long, value_name = "SOURCE")]
    source: Option<String>,

    /// Filter messages containing pattern (case-insensitive)
    #[arg(short = 'm', long, value_name = "PATTERN")]
    message: Option<String>,

    /// Include entries from this datetime (RFC3339, e.g. 2024-01-15T09:00:00Z)
    #[arg(long, value_name = "DATETIME")]
    from: Option<String>,

    /// Include entries up to this datetime (RFC3339)
    #[arg(long, value_name = "DATETIME")]
    to: Option<String>,

    /// Output format
    #[arg(short, long, value_name = "FORMAT", default_value = "table")]
    output: String,

    /// Show summary statistics instead of log entries
    #[arg(long)]
    stats: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load entries
    let analyzer = if cli.files.is_empty() {
        LogAnalyzer::load_stdin()?
    } else {
        LogAnalyzer::load_files(&cli.files)?
    };

    // Apply filters (builder pattern)
    let mut analyzer = analyzer;

    if let Some(ref level_str) = cli.level {
        let level = LogLevel::from_str(level_str)
            .map_err(|_| anyhow::anyhow!("Invalid level: {}. Use trace/debug/info/warn/error/fatal", level_str))?;
        analyzer = analyzer.filter_by_level(level);
    }

    if let Some(ref src) = cli.source {
        analyzer = analyzer.filter_by_source(src.clone());
    }

    if let Some(ref pattern) = cli.message {
        analyzer = analyzer.filter_by_message(pattern.clone());
    }

    if cli.from.is_some() || cli.to.is_some() {
        use chrono::{DateTime, Utc};
        let from = cli
            .from
            .as_deref()
            .map(|s| {
                s.parse::<DateTime<Utc>>()
                    .map_err(|_| anyhow::anyhow!("Invalid --from datetime: {}", s))
            })
            .transpose()?
            .unwrap_or(DateTime::<Utc>::MIN_UTC);

        let to = cli
            .to
            .as_deref()
            .map(|s| {
                s.parse::<DateTime<Utc>>()
                    .map_err(|_| anyhow::anyhow!("Invalid --to datetime: {}", s))
            })
            .transpose()?
            .unwrap_or(DateTime::<Utc>::MAX_UTC);

        analyzer = analyzer.filter_by_time_range(from, to);
    }

    let result = analyzer.analyze();

    // Render output
    let fmt = OutputFormat::from_str(&cli.output)?;

    if cli.stats {
        print!("{}", logagg::output::table::render_stats(&result)?);
    } else {
        print!("{}", output::render(&result, fmt)?);
    }

    Ok(())
}
