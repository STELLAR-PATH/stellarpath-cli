use crate::models::{ComponentType, Evidence};
use std::fs;
use std::path::Path;
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
        let path_str = file
            .strip_prefix(root_path)
            .unwrap_or(file)
            .to_string_lossy()
            .to_string();

        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            if ext == "rs" {
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
                                path: path_str.clone(),
                                detector_name: "SecurityLint".to_string(),
                                reason: format!(
                                    "Line {}: Mistake #17 (Bare panic! instead of typed errors)",
                                    i.saturating_add(1)
                                ),
                                confidence_bps: 10000,
                            });
                        }
                    }

                    // Mistake #18: Unsafe unwrap() / expect()
                    for (i, line) in lines.iter().enumerate() {
                        if line.contains(".unwrap()") || line.contains(".expect(") {
                            findings.push(Evidence {
                                component_type: ComponentType::SorobanContract,
                                path: path_str.clone(),
                                detector_name: "SecurityLint".to_string(),
                                reason: format!(
                                    "Line {}: Mistake #18 (Unsafe unwrap() / expect())",
                                    i.saturating_add(1)
                                ),
                                confidence_bps: 10000,
                            });
                        }
                    }

                    // Mistake #19: Missing events
                    if !content.contains("env.events().publish") && !content.contains(".publish(") {
                        findings.push(Evidence {
                            component_type: ComponentType::SorobanContract,
                            path: path_str.clone(),
                            detector_name: "SecurityLint".to_string(),
                            reason: "Mistake #19 (Missing events): No event publishing found in contract".to_string(),
                            confidence_bps: 7000,
                        });
                    }

                    // Storage Key Collisions: Flagging raw `symbol_short!` passed into `.set()`
                    for (i, line) in lines.iter().enumerate() {
                        if line.contains(".set(") && line.contains("symbol_short!(") {
                            findings.push(Evidence {
                                component_type: ComponentType::SorobanContract,
                                path: path_str.clone(),
                                detector_name: "SecurityLint".to_string(),
                                reason: format!("Line {}: Storage Key Collision (Raw symbol_short! passed into .set())", i.saturating_add(1)),
                                confidence_bps: 10000,
                            });
                        }
                    }
                }
            }
        }
    }

    for file in &files {
        let path_str = file
            .strip_prefix(root_path)
            .unwrap_or(file)
            .to_string_lossy()
            .to_string();
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            if ext == "rs" {
                if let Ok(content) = fs::read_to_string(file) {
                    let has_storage_access =
                        content.contains(".persistent()") || content.contains(".instance()");
                    let has_extend_ttl = content.contains(".extend_ttl(");
                    if has_storage_access && !has_extend_ttl {
                        findings.push(Evidence {
                            component_type: ComponentType::SorobanContract,
                            path: path_str.clone(),
                            detector_name: "SecurityLint".to_string(),
                            reason:
                                "Missing TTL extension (Storage accessed but extend_ttl not called)"
                                    .to_string(),
                            confidence_bps: 10000,
                        });
                    }
                }
            }
        }
    }

    for file in &files {
        let path_str = file
            .strip_prefix(root_path)
            .unwrap_or(file)
            .to_string_lossy()
            .to_string();
        if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
            if ext == "rs" || ext == "ts" || ext == "js" {
                if let Ok(content) = fs::read_to_string(file) {
                    let lines: Vec<&str> = content.lines().collect();
                    for (i, line) in lines.iter().enumerate() {
                        if line.contains("horizon.stellar.org")
                            || line.contains("StellarSdk.Server(")
                        {
                            findings.push(Evidence {
                                component_type: ComponentType::StellarSdk,
                                path: path_str.clone(),
                                detector_name: "SecurityLint".to_string(),
                                reason: format!("Line {}: Legacy Horizon RPC detected. Use soroban-rpc instead.", i.saturating_add(1)),
                                confidence_bps: 10000,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(LintResult { findings })
}
// TTL extension checks: Flagging persistent/instance storage access without corresponding TTL lifecycle extension
// Horizon RPC Rule
