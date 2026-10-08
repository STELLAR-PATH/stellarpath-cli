<div align="center">

# `stellarpath-cli`

**Deterministic Static Analysis & Security Linting Engine for Soroban Contracts**

[![Stellar Ecosystem](https://img.shields.io/badge/Stellar-Soroban-7B3FE4?style=for-the-badge&logo=stellar)](https://stellar.org)
[![Rust 2021](https://img.shields.io/badge/Rust-2021-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Drips Stellar Wave](https://img.shields.io/badge/Drips-Stellar%20Wave%20Participant-00D395?style=for-the-badge)](https://drips.network)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)

</div>

## 📖 Overview

`stellarpath-cli` is a high-performance Abstract Syntax Tree (AST) static analyzer built in Rust. It statically inspects Soroban smart contracts for security anti-patterns, storage collisions, and RPC hygiene without requiring a runtime WASM execution environment.

This tool acts as the core engine in the STELLAR-PATH ecosystem, providing deterministic security verification for Soroban developers.

## ✨ Key Features

- **Typed DataKey Collision Prevention**: Detects raw symbol usage that could lead to storage overlaps.
- **TTL Lifecycle Checks**: Ensures instance and persistent storage TTLs are correctly initialized and extended.
- **RPC Modernization**: Flags deprecated Horizon API usage in favor of modern Soroban RPCs.
- **Security Anti-Patterns**: Automatically detects common Soroban security mistakes (e.g., #17 panics, #18 unwrap abuses, #19 missing events).
- **Deterministic Traversal**: Powered by the Rust `syn` crate for fast, accurate AST traversal.

## 🚀 Installation

```bash
# Clone the repository
git clone https://github.com/STELLAR-PATH/stellarpath-cli.git
cd stellarpath-cli

# Build the release binary
cargo build --release

# (Optional) Move to your PATH
mv target/release/stellarpath ~/.local/bin/
```

## 🛠️ Usage

Run the static inspector against your Soroban contract source files:

```bash
stellarpath scan ./contracts --format terminal
```

Output formats supported: `terminal`, `json`, `sarif`.

## 🤝 Contributing & Reviewers

We welcome community contributions! This project is critical for the Drips Stellar Wave ecosystem.

**For Contributors:**
- The engine uses the `syn` crate. To add a new lint rule, implement the `Visitor` trait in the `src/lints/` directory.
- Please ensure `cargo fmt` and `cargo clippy` pass cleanly.
- Write unit tests for all new AST matchers in `tests/`.

**For Reviewers:**
- All AST evaluations are deterministic. When reviewing PRs, verify that the edge-case unit tests are exhaustive for the Soroban contract syntax being targeted.

---
<div align="center">
  <sub>Part of the <a href="https://github.com/STELLAR-PATH">STELLAR-PATH</a> Toolchain. Built for the Soroban ecosystem.</sub>
</div>
