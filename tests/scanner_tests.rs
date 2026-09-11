use std::fs;
use std::path::PathBuf;
use stellarpath::scanner::{ScanConfig, Scanner};
use tempfile::tempdir;

#[test]
fn test_scan_empty_directory() {
    let dir = tempdir().unwrap();
    let scanner = Scanner::new(ScanConfig::default());
    let results = scanner.scan(dir.path());
    assert!(results.is_empty());
}

#[test]
fn test_scan_nested_structure() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("src/nested")).unwrap();
    fs::write(root.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(root.join("src/nested/lib.rs"), "pub fn hello() {}").unwrap();
    fs::write(root.join("Cargo.toml"), "[package]").unwrap();

    let scanner = Scanner::new(ScanConfig::default());
    let mut results = scanner.scan(root);
    results.sort();

    let mut expected = vec![
        PathBuf::from("Cargo.toml"),
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/nested/lib.rs"),
    ];
    expected.sort();

    assert_eq!(results, expected);
}

#[test]
fn test_scan_excludes_directories() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir_all(root.join("target/debug")).unwrap();
    fs::write(root.join("target/debug/app"), "bin").unwrap();
    fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
    fs::write(root.join("node_modules/pkg/index.js"), "js").unwrap();
    fs::write(root.join("test.wasm"), "wasm").unwrap();
    fs::write(root.join("valid.rs"), "fn ok() {}").unwrap();

    let scanner = Scanner::new(ScanConfig::default());
    let results = scanner.scan(root);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0], PathBuf::from("valid.rs"));
}
