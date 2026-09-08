use std::path::PathBuf;
use stellar_path::detector::registry::DetectorRegistry;
use stellar_path::detector::ScanContext;
use stellar_path::models::ComponentType;

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
