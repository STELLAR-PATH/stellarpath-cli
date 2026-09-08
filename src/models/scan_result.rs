use super::project_model::ProjectModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanResult {
    pub project: ProjectModel,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
