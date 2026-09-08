use std::path::{Path, PathBuf};
use std::time::Duration;
use stellar_path::analyzer::Analyzer;
use stellar_path::models::{ComponentType, Evidence};

#[test]
fn test_classify_archetype_soroban_only() {
    let evidence = vec![Evidence {
        component_type: ComponentType::SorobanContract,
        path: "src/lib.rs".to_string(),
        detector_name: "SorobanDetector".to_string(),
        reason: "".to_string(),
        confidence: 1.0,
    }];
    let files = vec![PathBuf::from("src/lib.rs")];
    let model = Analyzer::analyze(
        Path::new("dummy"),
        &files,
        &evidence,
        Duration::from_secs(1),
    );
    assert_eq!(model.archetype.unwrap(), "Soroban Smart Contract");
}

#[test]
fn test_classify_archetype_sdk_only() {
    let evidence = vec![Evidence {
        component_type: ComponentType::StellarSdk,
        path: "package.json".to_string(),
        detector_name: "StellarSdkDetector".to_string(),
        reason: "".to_string(),
        confidence: 1.0,
    }];
    let files = vec![PathBuf::from("package.json")];
    let model = Analyzer::analyze(
        Path::new("dummy"),
        &files,
        &evidence,
        Duration::from_secs(1),
    );
    assert_eq!(model.archetype.unwrap(), "Stellar DApp");
}

#[test]
fn test_classify_archetype_monorepo() {
    let evidence = vec![
        Evidence {
            component_type: ComponentType::SorobanContract,
            path: "contracts/hello/src/lib.rs".to_string(),
            detector_name: "SorobanDetector".to_string(),
            reason: "".to_string(),
            confidence: 1.0,
        },
        Evidence {
            component_type: ComponentType::StellarSdk,
            path: "frontend/package.json".to_string(),
            detector_name: "StellarSdkDetector".to_string(),
            reason: "".to_string(),
            confidence: 1.0,
        },
    ];
    let files = vec![
        PathBuf::from("contracts/hello/src/lib.rs"),
        PathBuf::from("frontend/package.json"),
    ];
    let model = Analyzer::analyze(
        Path::new("dummy"),
        &files,
        &evidence,
        Duration::from_secs(1),
    );
    assert_eq!(
        model.archetype.unwrap(),
        "Full-Stack Stellar / Soroban Monorepo"
    );
}

#[test]
fn test_recommendation_ordering() {
    let evidence = vec![Evidence {
        component_type: ComponentType::SorobanContract,
        path: "src/lib.rs".to_string(),
        detector_name: "SorobanDetector".to_string(),
        reason: "".to_string(),
        confidence: 1.0,
    }];
    let files = vec![
        PathBuf::from("README.md"),
        PathBuf::from("src/lib.rs"),
        PathBuf::from("Cargo.toml"),
    ];

    let model = Analyzer::analyze(
        Path::new("dummy"),
        &files,
        &evidence,
        Duration::from_secs(1),
    );

    assert_eq!(model.recommendations.len(), 3);
    assert_eq!(model.recommendations[0].step, 1);
    assert_eq!(model.recommendations[0].path, "README.md");

    assert_eq!(model.recommendations[1].step, 2);
    assert_eq!(model.recommendations[1].path, "src/lib.rs");

    assert_eq!(model.recommendations[2].step, 3);
    assert_eq!(model.recommendations[2].path, "Cargo.toml");
}
