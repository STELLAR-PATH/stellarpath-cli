use stellarpath::lint::run_lint;

#[test]
fn test_security_lint() {
    let root = std::env::current_dir()
        .unwrap()
        .join("testdata/lint-project");
    let result = run_lint(&root).unwrap();
    let has_panic = result
        .findings
        .iter()
        .any(|e| e.reason.contains("Bare panic"));
    assert!(has_panic, "Should detect bare panic");

    let has_collision = result
        .findings
        .iter()
        .any(|e| e.reason.contains("Storage Key Collision"));
    assert!(has_collision, "Should detect storage key collision");

    let has_ttl = result
        .findings
        .iter()
        .any(|e| e.reason.contains("Missing TTL extension"));
    assert!(has_ttl, "Should detect missing TTL extension");

    let has_horizon = result
        .findings
        .iter()
        .any(|e| e.reason.contains("Legacy Horizon RPC"));
    assert!(has_horizon, "Should detect Legacy Horizon RPC");
}
