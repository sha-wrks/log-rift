use anyhow::Result;
use prettytable::{Cell, Row, Table};

use crate::analyzer::AnalysisResult;

pub fn render(result: &AnalysisResult) -> Result<String> {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Timestamp"),
        Cell::new("Level"),
        Cell::new("Source"),
        Cell::new("Message"),
    ]));

    for entry in &result.entries {
        let ts = entry
            .timestamp
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "-".to_string());

        let msg = if entry.message.len() > 80 {
            format!("{}…", &entry.message[..79])
        } else {
            entry.message.clone()
        };

        table.add_row(Row::new(vec![
            Cell::new(&ts),
            Cell::new(&entry.level.to_string()),
            Cell::new(&entry.source),
            Cell::new(&msg),
        ]));
    }

    Ok(format!("{table}"))
}

pub fn render_stats(result: &AnalysisResult) -> Result<String> {
    let mut output = String::new();

    output.push_str(&format!("Total entries : {}\n", result.count));
    output.push_str(&format!("Error rate    : {:.1}%\n\n", result.error_rate));

    output.push_str("By level:\n");
    let mut levels: Vec<_> = result.level_counts.iter().collect();
    levels.sort_by(|a, b| b.1.cmp(a.1));
    for (level, count) in &levels {
        output.push_str(&format!("  {:8} {}\n", level, count));
    }

    output.push_str("\nBy source (top 10):\n");
    let mut sources: Vec<_> = result.source_counts.iter().collect();
    sources.sort_by(|a, b| b.1.cmp(a.1));
    for (src, count) in sources.iter().take(10) {
        output.push_str(&format!("  {:30} {}\n", src, count));
    }

    Ok(output)
}
