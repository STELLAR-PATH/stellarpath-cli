use crate::models::ScanResult;
use crate::report::ReportRenderer;

pub struct JsonRenderer;

impl ReportRenderer for JsonRenderer {
    fn render(&self, result: &ScanResult) -> Result<String, Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(result)?;
        Ok(json)
    }
}
