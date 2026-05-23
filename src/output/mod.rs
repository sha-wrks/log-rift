pub mod csv;
pub mod json;
pub mod table;

use anyhow::Result;

use crate::analyzer::AnalysisResult;

pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(anyhow::anyhow!("Unknown format: {}. Use table, json, or csv.", s)),
        }
    }
}

pub fn render(result: &AnalysisResult, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Table => table::render(result),
        OutputFormat::Json => json::render(result),
        OutputFormat::Csv => csv::render(result),
    }
}
