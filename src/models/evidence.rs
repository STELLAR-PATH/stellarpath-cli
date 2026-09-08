use super::component_type::ComponentType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {
    pub component_type: ComponentType,
    pub path: String,
    pub detector_name: String,
    pub reason: String,
    pub confidence: f32,
}
