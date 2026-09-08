pub mod registry;
pub mod soroban;
pub mod stellar_sdk;

use crate::models::Evidence;
use std::path::{Path, PathBuf};

pub struct ScanContext<'a> {
    pub root_path: &'a Path,
    pub files: &'a [PathBuf],
}

pub trait Detector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>>;
}
