use super::evidence::Evidence;
use super::recommendation::Recommendation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectModel {
    pub name: String,
    pub root_path: String,
    pub archetype: Option<String>,
    pub languages: Vec<String>,
    pub evidence: Vec<Evidence>,
    pub important_files: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub scan_duration: f64,
}
