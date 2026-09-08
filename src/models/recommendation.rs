use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recommendation {
    pub step: usize,
    pub path: String,
    pub title: String,
    pub reason: String,
    pub suggested_action: Option<String>,
}
