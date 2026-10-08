<div align="center">

```text
███████╗████████╗███████╗██╗     ██╗      █████╗ ██████╗       ██████╗  █████╗ ████████╗██╗  ██╗
██╔════╝╚══██╔══╝██╔════╝██║     ██║     ██╔══██╗██╔══██╗      ██╔══██╗██╔══██╗╚══██╔══╝██║  ██║
███████╗   ██║   █████╗  ██║     ██║     ███████║██████╔╝█████╗██████╔╝███████║   ██║   ███████║
╚════██║   ██║   ██╔══╝  ██║     ██║     ██╔══██║██╔══██╗╚════╝██╔═══╝ ██╔══██║   ██║   ██╔══██║
███████║   ██║   ███████╗███████╗███████╗██║  ██║██║  ██║      ██║     ██║  ██║   ██║   ██║  ██║
╚══════╝   ╚═╝   ╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝      ╚═╝     ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
```

# **stellarpath-cli**
### Deterministic AST Static Analysis Engine for Soroban

[![Stellar Ecosystem](https://img.shields.io/badge/Stellar-Soroban-7B3FE4?style=for-the-badge&logo=stellar)](https://stellar.org)
[![Rust 2021](https://img.shields.io/badge/Rust-2021-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Drips Stellar Wave](https://img.shields.io/badge/Drips-Stellar%20Wave%20Participant-00D395?style=for-the-badge)](https://drips.network)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)

</div>

---

## 📖 1. Executive Summary

`stellarpath-cli` is an industrial-grade, zero-runtime Abstract Syntax Tree (AST) static analysis tool explicitly designed for the Soroban smart contract ecosystem. Unlike traditional linters that rely on regex or dynamic execution trace analysis, this engine strictly leverages the Rust `syn` crate to deterministically traverse contract source code.

Inspired by standards set by **StellarCanary** and **SoroTrail**, `stellarpath-cli` targets common Soroban security anti-patterns (such as raw DataKey symbol collisions, lifecycle TTL mismanagement, and insecure RPC endpoint usage) to prevent vulnerabilities prior to WASM compilation.

---

## 🏗️ 2. Core Architecture & Determinism

### The `syn` Traversal Pipeline
The core loop operates exclusively on parsed AST nodes rather than raw strings:
1. **Tokenization**: Source files are parsed via `proc_macro2` and `syn`.
2. **Visitor Implementation**: Custom `syn::visit::Visit` trait implementations traverse `ItemFn`, `ItemEnum`, and `Macro` nodes.
3. **Deterministic Evaluation**: Each visitor applies exact structural pattern matching. If an `env.storage().instance().set(...)` call doesn't enforce a typed `enum` key, it deterministically flags the line.

```text
       +-------------------------------------------------------------+
       |                  stellarpath-cli (Rust Engine)              |
       |  * Abstract Syntax Tree (AST) Traversal (`syn`)             |
       |  * Typed DataKey collision prevention                       |
       |  * Instance / Persistent Storage TTL lifecycle checks       |
       |  * Modern RPC vs Horizon endpoint detection                 |
       |  * Security Mistakes (#17 panic, #18 unwrap, #19 events)    |
       +-------------------------------------------------------------+
```

---

## 🚀 3. Installation Specifications

### Method A: Cargo (Recommended)
Compile the engine natively using the stable Rust toolchain.
```bash
cargo install --git https://github.com/STELLAR-PATH/stellarpath-cli.git
```

### Method B: Source Build
```bash
git clone https://github.com/STELLAR-PATH/stellarpath-cli.git
cd stellarpath-cli
cargo build --release
sudo cp target/release/stellarpath /usr/local/bin/
```

### Method C: Docker Image
```bash
docker pull ghcr.io/stellar-path/stellarpath-cli:latest
docker run -v $(pwd):/workspace ghcr.io/stellar-path/stellarpath-cli scan /workspace
```

---

## ⌨️ 4. CLI Command Matrix

| Command | Flags / Arguments | Description | Output Format |
| :--- | :--- | :--- | :--- |
| `scan` | `<DIR> --format [term,json,sarif]` | Executes the primary AST analysis traversal over all `.rs` files in the target directory. | Terminal, JSON, SARIF |
| `tree` | `<DIR> --depth <N>` | Outputs a structural map of the repository, identifying contract entrypoints and dependencies. | ASCII Tree |
| `doctor` | `--fix` | Analyzes `Cargo.toml` and `.cargo/config.toml` to verify Soroban SDK pinning and optimization flags. | Terminal |
| `explain`| `[ERROR_CODE]` | Provides long-form, detailed explanations of specific security anti-patterns and remediation steps. | Markdown |

### Configuration: `stellarpath.toml`
Place a `stellarpath.toml` in your repository root to configure the engine:
```toml
[core]
strict_mode = true          # Fails CI on ANY warning
exclude_dirs = ["tests/", "benches/"]

[rules]
e0001_raw_datakey = "deny"
e0002_missing_ttl = "warn"
e0003_unwrap_used = "allow"
```

---

## 🛡️ 5. Error Codes & Security Diagnostics

`stellarpath-cli` assigns unique diagnostic codes to every detected anti-pattern.

### `E0001`: Raw Symbol DataKey Collision
Using raw `Symbol::short("admin")` directly in storage operations can lead to unintended collisions in complex contracts.

**❌ Vulnerable Code:**
```rust
env.storage().instance().set(&Symbol::short("admin"), &admin_address);
```

**✅ Remediated Code:**
```rust
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Allowance(Address),
}

env.storage().instance().set(&DataKey::Admin, &admin_address);
```

### `E0002`: Missing Storage TTL Extension
Soroban state requires TTL extensions. Writing state without subsequently extending its TTL is flagged as a high-severity risk.

**❌ Vulnerable Code:**
```rust
env.storage().persistent().set(&DataKey::Balance, &amount);
// Missing extend_ttl call
```

### `E0003`: Insecure RPC/Horizon Overlap
Flagging legacy Horizon endpoints when modern Soroban RPC endpoints should be used for data indexing.

---

## 🤝 6. Contributing Guidelines

We enforce a strict development standard for `stellarpath-cli`.

1. **New Lint Rules**: Must implement `syn::visit::Visit`. Create a new module in `src/lints/` and register it in the master visitor registry.
2. **Unit Testing**: You must provide exhaustive positive and negative test cases utilizing raw string parsing: `syn::parse_str::<syn::File>(&code)`.
3. **Format & Clippy**: `cargo fmt --all -- --check` and `cargo clippy --all-targets -- -D warnings` must pass.

---
<div align="center">
  <sub>Part of the <b>STELLAR-PATH</b> Toolchain. Built for the Soroban ecosystem.</sub>
</div>
