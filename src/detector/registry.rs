use super::soroban::SorobanDetector;
use super::stellar_sdk::StellarSdkDetector;
use super::{Detector, ScanContext};
use crate::models::Evidence;

#[derive(Default)]
pub struct DetectorRegistry {
    detectors: Vec<Box<dyn Detector>>,
}

impl DetectorRegistry {
    pub fn new() -> Self {
        Self {
            detectors: Vec::new(),
        }
    }

    pub fn register<D: Detector + 'static>(&mut self, detector: D) {
        self.detectors.push(Box::new(detector));
    }

    pub fn default_registry() -> Self {
        let mut registry = Self::new();
        registry.register(SorobanDetector);
        registry.register(StellarSdkDetector);
        registry
    }

    pub fn run_all(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
        let mut all_evidence = Vec::new();
        for detector in &self.detectors {
            let mut evidence = detector.detect(ctx)?;
            all_evidence.append(&mut evidence);
        }
        Ok(all_evidence)
    }
}
