use std::path::PathBuf;
use stellarpath::detector::registry::DetectorRegistry;
use stellarpath::detector::ScanContext;
use stellarpath::models::ComponentType;

#[test]
fn test_soroban_detector_matches_fixture() {
    let root = std::env::current_dir()
        .unwrap()
        .join("testdata/soroban-project");
    let files = vec![PathBuf::from("Cargo.toml"), PathBuf::from("src/lib.rs")];

    let ctx = ScanContext {
        root_path: &root,
        files: &files,
    };

    let registry = DetectorRegistry::default_registry();
    let evidence = registry.run_all(&ctx).unwrap();

    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].component_type, ComponentType::SorobanContract);
    assert_eq!(evidence[0].path, "src/lib.rs");
    assert_eq!(evidence[0].confidence, 1.0);
}

#[test]
fn test_stellar_sdk_detector_matches_fixture() {
    let root = std::env::current_dir()
        .unwrap()
        .join("testdata/stellar-sdk-project");
    let files = vec![PathBuf::from("package.json")];

    let ctx = ScanContext {
        root_path: &root,
        files: &files,
    };

    let registry = DetectorRegistry::default_registry();
    let evidence = registry.run_all(&ctx).unwrap();

    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].component_type, ComponentType::StellarSdk);
    assert_eq!(evidence[0].path, "package.json");
    assert_eq!(evidence[0].confidence, 1.0);
}

#[test]
fn test_sep_detector_matches_fixture() {
    let root = std::env::current_dir()
        .unwrap()
        .join("testdata/sep-project");
    let files = vec![PathBuf::from("src/index.ts")];

    let ctx = ScanContext {
        root_path: &root,
        files: &files,
    };

    let registry = DetectorRegistry::default_registry();
    let evidence = registry.run_all(&ctx).unwrap();

    // It should find SEP-10, SEP-24, and SEP-53 evidence
    assert!(evidence.len() >= 3, "Expected at least 3 pieces of evidence for SEPs");
    
    let has_sep10 = evidence.iter().any(|e| e.reason.contains("SEP-10"));
    let has_sep24 = evidence.iter().any(|e| e.reason.contains("SEP-24"));
    let has_sep53 = evidence.iter().any(|e| e.reason.contains("SEP-53"));

    assert!(has_sep10, "Should detect SEP-10");
    assert!(has_sep24, "Should detect SEP-24");
    assert!(has_sep53, "Should detect SEP-53");
}
