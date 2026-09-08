use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    SorobanContract,
    StellarSdk,
    TestSuite,
    Configuration,
    Documentation,
}
