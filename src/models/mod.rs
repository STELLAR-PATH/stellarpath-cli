pub mod component_type;
pub mod evidence;
pub mod project_model;
pub mod recommendation;
pub mod scan_result;

pub use component_type::ComponentType;
pub use evidence::Evidence;
pub use project_model::ProjectModel;
pub use recommendation::Recommendation;
pub use scan_result::ScanResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_result_serialization() {
        let result = ScanResult {
            project: ProjectModel {
                name: "test_project".to_string(),
                root_path: "/test".to_string(),
                archetype: Some("soroban".to_string()),
                languages: vec!["rust".to_string()],
                evidence: vec![Evidence {
                    component_type: ComponentType::SorobanContract,
                    path: "src/contract.rs".to_string(),
                    detector_name: "contract_detector".to_string(),
                    reason: "found contract".to_string(),
                    confidence: 1.0,
                }],
                important_files: vec!["Cargo.toml".to_string()],
                recommendations: vec![Recommendation {
                    step: 1,
                    path: "Cargo.toml".to_string(),
                    title: "Update".to_string(),
                    reason: "Old version".to_string(),
                    suggested_action: Some("bump version".to_string()),
                }],
                scan_duration: 1.5,
            },
            warnings: vec!["warning1".to_string()],
            errors: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ScanResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result, deserialized);
    }
}
