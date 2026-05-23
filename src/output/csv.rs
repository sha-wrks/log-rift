use anyhow::Result;

use crate::analyzer::AnalysisResult;

pub fn render(result: &AnalysisResult) -> Result<String> {
    let mut out = String::from("timestamp,level,source,message\n");

    for entry in &result.entries {
        let ts = entry
            .timestamp
            .map(|t| t.to_rfc3339())
            .unwrap_or_default();

        let msg = entry.message.replace('"', "\"\"");

        out.push_str(&format!(
            "{},{},{},\"{}\"\n",
            ts, entry.level, entry.source, msg
        ));
    }

    Ok(out)
}
