use stellarpath::lint::run_lint;
use std::path::PathBuf;

#[test]
fn test_security_lint() {
    let root = std::env::current_dir().unwrap().join("testdata/lint-project");
    let result = run_lint(&root).unwrap();
    let has_panic = result.findings.iter().any(|e| e.reason.contains("Bare panic"));
    assert!(has_panic, "Should detect bare panic");
}
