use crate::detector::{Detector, ScanContext};
use crate::models::{ComponentType, Evidence};
use std::fs;

pub struct SepDetector;

impl Detector for SepDetector {
    fn name(&self) -> &'static str {
        "SepDetector"
    }

    fn detect(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
        let mut evidence = Vec::new();

        for file in ctx.files {
            if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "ts" | "js" | "go" | "py" | "rs") {
                    let full_path = ctx.root_path.join(file);
                    if let Ok(content) = fs::read_to_string(&full_path) {
                        if content.contains("buildChallengeTransaction") || content.contains("verifyChallengeTransaction") || content.contains("SEP-10") || content.contains("SEP10") {
                            evidence.push(Evidence {
                                component_type: ComponentType::StellarSdk, // or define a new one if available
                                path: file.to_string_lossy().to_string(),
                                detector_name: self.name().to_string(),
                                reason: "Implements SEP-10 (Stellar Web Authentication)".to_string(),
                                confidence: 0.9,
                            });
                        }
                        if content.contains("/transactions/deposit/interactive") || content.contains("/transactions/withdraw/interactive") || content.contains("SEP-24") || content.contains("SEP24") {
                            evidence.push(Evidence {
                                component_type: ComponentType::StellarSdk,
                                path: file.to_string_lossy().to_string(),
                                detector_name: self.name().to_string(),
                                reason: "Implements SEP-24 (Hosted Deposit and Withdrawal)".to_string(),
                                confidence: 0.9,
                            });
                        }
                        if content.contains("SEP-53") || content.contains("SEP53") || content.contains("Sign-In with Stellar") {
                            evidence.push(Evidence {
                                component_type: ComponentType::StellarSdk,
                                path: file.to_string_lossy().to_string(),
                                detector_name: self.name().to_string(),
                                reason: "Implements SEP-53 (Sign-In with Stellar)".to_string(),
                                confidence: 0.9,
                            });
                        }
                    }
                }
            }
        }

        Ok(evidence)
    }
}
