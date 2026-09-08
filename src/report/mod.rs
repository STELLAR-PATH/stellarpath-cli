pub mod json;
pub mod markdown;
pub mod terminal;

use crate::models::ScanResult;

pub trait ReportRenderer {
    fn render(&self, result: &ScanResult) -> Result<String, Box<dyn std::error::Error>>;
}
