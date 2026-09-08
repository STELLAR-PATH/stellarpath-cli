pub mod analyzer;
pub mod detector;
pub mod models;
pub mod report;
pub mod scanner;

use crate::analyzer::Analyzer;
use crate::detector::registry::DetectorRegistry;
use crate::detector::ScanContext;
use crate::models::ScanResult;
use crate::scanner::{ScanConfig, Scanner};
use std::path::Path;
use std::time::Instant;

pub fn run_scan(path: &Path, config: ScanConfig) -> Result<ScanResult, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let scanner = Scanner::new(config);
    let files = scanner.scan(path);

    let registry = DetectorRegistry::default_registry();
    let ctx = ScanContext {
        root_path: path,
        files: &files,
    };
    let evidence = registry.run_all(&ctx)?;

    let project = Analyzer::analyze(path, &files, &evidence, start_time.elapsed());

    Ok(ScanResult {
        project,
        warnings: vec![],
        errors: vec![],
    })
}
