use anyhow::Result;

use crate::analyzer::AnalysisResult;

pub fn render(result: &AnalysisResult) -> Result<String> {
    Ok(serde_json::to_string_pretty(result)?)
}
