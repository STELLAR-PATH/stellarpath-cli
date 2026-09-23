use crate::models::{ComponentType, Evidence};
use std::fs;
use std::path::{Path};
use walkdir::WalkDir;

pub struct LintResult {
    pub findings: Vec<Evidence>,
}

pub fn run_lint(root_path: &Path) -> Result<LintResult, Box<dyn std::error::Error>> {
    let mut findings = Vec::new();

    let mut files = Vec::new();
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    for file in &files {
        if file.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(file) {
                // Check if it's a contract
                if !content.contains("#[contract]") && !content.contains("#[contractimpl]") {
                    continue;
                }

                let lines: Vec<&str> = content.lines().collect();

                // Mistake #17: Bare panic! instead of typed errors
                for (i, line) in lines.iter().enumerate() {
                    if line.contains("panic!(") && !line.contains("panic_with_error!(") {
                        findings.push(Evidence {
                            component_type: ComponentType::SorobanContract,
                            path: file.strip_prefix(root_path).unwrap_or(file).to_string_lossy().to_string(),
                            detector_name: "SecurityLint".to_string(),
                            reason: format!("Line {}: Mistake #17 (Bare panic! instead of typed errors)", i + 1),
                            confidence: 1.0,
                        });
                    }
                }

                // Mistake #18: Unsafe unwrap() / expect()
                for (i, line) in lines.iter().enumerate() {
                    if line.contains(".unwrap()") || line.contains(".expect(") {
                        findings.push(Evidence {
                            component_type: ComponentType::SorobanContract,
                            path: file.strip_prefix(root_path).unwrap_or(file).to_string_lossy().to_string(),
                            detector_name: "SecurityLint".to_string(),
                            reason: format!("Line {}: Mistake #18 (Unsafe unwrap() / expect())", i + 1),
                            confidence: 1.0,
                        });
                    }
                }

                // Mistake #19: Missing events
                // This is slightly heuristic, but if there are state changing operations and no events, it's a warning.
                // We'll skip complex heuristics and stick to static facts. If the whole file lacks env.events().publish
                if !content.contains("env.events().publish") && !content.contains(".publish(") {
                    findings.push(Evidence {
                        component_type: ComponentType::SorobanContract,
                        path: file.strip_prefix(root_path).unwrap_or(file).to_string_lossy().to_string(),
                        detector_name: "SecurityLint".to_string(),
                        reason: "Mistake #19 (Missing events): No event publishing found in contract".to_string(),
                        confidence: 0.7, // A bit of a heuristic for the whole file
                    });
                }
            }
        }
    }

    Ok(LintResult { findings })
}
