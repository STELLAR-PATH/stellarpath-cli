use crate::detector::{Detector, ScanContext};
use crate::models::{ComponentType, Evidence};
use std::fs;

pub struct StellarSdkDetector;

impl Detector for StellarSdkDetector {
    fn name(&self) -> &'static str {
        "StellarSdkDetector"
    }

    fn detect(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
        let mut evidence = Vec::new();

        for file in ctx.files {
            let file_name = file.file_name().and_then(|n| n.to_str());
            let full_path = ctx.root_path.join(file);

            if file_name == Some("package.json") {
                if let Ok(content) = fs::read_to_string(&full_path) {
                    if content.contains("\"@stellar/stellar-sdk\"")
                        || content.contains("\"stellar-sdk\"")
                    {
                        evidence.push(Evidence {
                            component_type: ComponentType::StellarSdk,
                            path: file.to_string_lossy().to_string(),
                            detector_name: self.name().to_string(),
                            reason: "Found Stellar SDK in package.json".to_string(),
                            confidence: 1.0,
                        });
                    }
                }
            } else if file_name == Some("go.mod") {
                if let Ok(content) = fs::read_to_string(&full_path) {
                    if content.contains("github.com/stellar/go") {
                        evidence.push(Evidence {
                            component_type: ComponentType::StellarSdk,
                            path: file.to_string_lossy().to_string(),
                            detector_name: self.name().to_string(),
                            reason: "Found Stellar SDK in go.mod".to_string(),
                            confidence: 1.0,
                        });
                    }
                }
            } else if file_name == Some("requirements.txt") {
                if let Ok(content) = fs::read_to_string(&full_path) {
                    if content.contains("stellar-sdk") {
                        evidence.push(Evidence {
                            component_type: ComponentType::StellarSdk,
                            path: file.to_string_lossy().to_string(),
                            detector_name: self.name().to_string(),
                            reason: "Found Stellar SDK in requirements.txt".to_string(),
                            confidence: 1.0,
                        });
                    }
                }
            }
        }

        Ok(evidence)
    }
}
