use crate::models::{ComponentType, Evidence, ProjectModel, Recommendation};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(
        root_path: &Path,
        files: &[PathBuf],
        evidence: &[Evidence],
        duration: Duration,
    ) -> ProjectModel {
        let archetype = Self::classify_archetype(evidence);
        let languages = Self::discover_languages(files);
        let recommendations = Self::generate_recommendations(files, evidence);

        let mut important_set = std::collections::HashSet::new();
        for e in evidence {
            important_set.insert(e.path.clone());
        }
        for r in &recommendations {
            important_set.insert(r.path.clone());
        }

        let mut important_files: Vec<String> = important_set.into_iter().collect();
        important_files.sort();

        ProjectModel {
            name: root_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            root_path: root_path.to_string_lossy().to_string(),
            archetype: Some(archetype),
            languages,
            evidence: evidence.to_vec(),
            important_files,
            recommendations,
            scan_duration: duration.as_secs_f64(),
        }
    }

    fn discover_languages(files: &[PathBuf]) -> Vec<String> {
        let mut langs = std::collections::HashSet::new();
        for file in files {
            if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => {
                        langs.insert("Rust".to_string());
                    }
                    "ts" | "js" => {
                        langs.insert("TypeScript/JavaScript".to_string());
                    }
                    "go" => {
                        langs.insert("Go".to_string());
                    }
                    "py" => {
                        langs.insert("Python".to_string());
                    }
                    _ => {}
                }
            }
        }
        let mut langs_vec: Vec<String> = langs.into_iter().collect();
        langs_vec.sort();
        langs_vec
    }

    fn generate_recommendations(files: &[PathBuf], evidence: &[Evidence]) -> Vec<Recommendation> {
        let mut recs = Vec::new();
        let mut step = 1;

        if let Some(readme) = files.iter().find(|f| {
            f.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase()
                == "readme.md"
        }) {
            recs.push(Recommendation {
                step,
                path: readme.to_string_lossy().to_string(),
                title: "Review Documentation".to_string(),
                reason: "Start by reading the project documentation to understand its structure."
                    .to_string(),
                suggested_action: Some("Read this file".to_string()),
            });
            step += 1;
        }

        let has_soroban = evidence
            .iter()
            .any(|e| e.component_type == ComponentType::SorobanContract);
        if has_soroban {
            if let Some(contract_file) = evidence
                .iter()
                .find(|e| e.component_type == ComponentType::SorobanContract)
            {
                recs.push(Recommendation {
                    step,
                    path: contract_file.path.clone(),
                    title: "Explore Smart Contracts".to_string(),
                    reason: "Soroban smart contracts are a core part of this project.".to_string(),
                    suggested_action: Some("Review contract implementation".to_string()),
                });
                step += 1;
            }
        }

        let has_sdk = evidence
            .iter()
            .any(|e| e.component_type == ComponentType::StellarSdk);
        if has_sdk {
            if let Some(sdk_evidence) = evidence
                .iter()
                .find(|e| e.component_type == ComponentType::StellarSdk)
            {
                recs.push(Recommendation {
                    step,
                    path: sdk_evidence.path.clone(),
                    title: "Check Client/SDK Integration".to_string(),
                    reason: "The project uses the Stellar SDK for client integrations.".to_string(),
                    suggested_action: Some("Review SDK usage".to_string()),
                });
                step += 1;
            }
        }

        let has_cargo = files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml"));
        let has_npm = files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("package.json"));

        if has_cargo {
            recs.push(Recommendation {
                step,
                path: "Cargo.toml".to_string(),
                title: "Run Tests".to_string(),
                reason: "A Rust project was detected.".to_string(),
                suggested_action: Some("cargo test".to_string()),
            });
        } else if has_npm {
            recs.push(Recommendation {
                step,
                path: "package.json".to_string(),
                title: "Run Tests".to_string(),
                reason: "A Node.js project was detected.".to_string(),
                suggested_action: Some("npm test".to_string()),
            });
        }

        recs
    }

    fn classify_archetype(evidence: &[Evidence]) -> String {
        let has_soroban = evidence
            .iter()
            .any(|e| e.component_type == ComponentType::SorobanContract);
        let has_sdk = evidence
            .iter()
            .any(|e| e.component_type == ComponentType::StellarSdk);

        match (has_soroban, has_sdk) {
            (true, true) => "Full-Stack Stellar / Soroban Monorepo".to_string(),
            (true, false) => "Soroban Smart Contract".to_string(),
            (false, true) => "Stellar DApp".to_string(),
            (false, false) => "Generic / Unknown Project".to_string(),
        }
    }
}
