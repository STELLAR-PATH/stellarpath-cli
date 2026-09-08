# Contributing to StellarPath

First off, thank you for considering contributing to StellarPath! 

## Getting Started

1. **Rust Toolchain**: Make sure you have the Rust 2021 edition toolchain installed. You can install it via [rustup](https://rustup.rs/).
2. **Clone the repository**:
   ```bash
   git clone https://github.com/STELLAR-PATH/stellarpath-cli.git
   cd stellarpath-cli
   ```
3. **Build the project**:
   ```bash
   cargo build
   ```
4. **Run tests**: We have a suite of tests against mocked repositories in the `testdata/` folder.
   ```bash
   cargo test
   ```

## Adding a Custom Detector

One of the best ways to contribute is by adding a new `Detector`. Detectors scan ASTs or configuration files to identify Stellar/Soroban components.

1. Create a new file in `src/detector/` (e.g., `my_detector.rs`).
2. Implement the `Detector` trait:
   ```rust
   use crate::detector::{Detector, ScanContext};
   use crate::models::Evidence;

   pub struct MyDetector;

   impl Detector for MyDetector {
       fn name(&self) -> &'static str {
           "MyDetector"
       }

       fn detect(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
           let mut evidence = Vec::new();
           // Implementation details...
           Ok(evidence)
       }
   }
   ```
3. Register your detector in `src/detector/registry.rs`.
4. Add relevant fixtures to `testdata/` and write a test in `tests/detector_tests.rs`.

## Pull Request Process

1. Ensure your code passes all checks: `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.
2. Push your branch and open a PR using our Pull Request Template.
3. A maintainer will review your code shortly.
