<div align="center">

<h1><code>stellarpath-cli</code></h1>
<h3>Deterministic AST Static Analysis Engine for Soroban</h3>

[![Stellar Ecosystem](https://img.shields.io/badge/Stellar-Soroban-7B3FE4?style=for-the-badge&logo=stellar)](https://stellar.org)
[![Rust 2021](https://img.shields.io/badge/Rust-2021-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)

</div>

---

## 1. Executive Summary

`stellarpath-cli` is an industrial-grade, zero-runtime Abstract Syntax Tree (AST) static analysis tool explicitly designed for the Soroban smart contract ecosystem. Unlike traditional linters that rely on regex or dynamic execution trace analysis, this engine strictly leverages the Rust `syn` crate to deterministically traverse contract source code.

Inspired by standards set by **StellarCanary** and **SoroTrail**, `stellarpath-cli` targets common Soroban security anti-patterns (such as raw DataKey symbol collisions, lifecycle TTL mismanagement, and insecure RPC endpoint usage) to prevent vulnerabilities prior to WASM compilation.

---

## 2. Core Architecture & Determinism

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

## 3. Installation Specifications

### Method A: Cargo (Recommended)
Compile the engine natively using the stable Rust toolchain.

- `cargo⠀install⠀--git⠀https://github.com/STELLAR-PATH/stellarpath-cli.git`


### Method B: Source Build

- `git⠀clone⠀https://github.com/STELLAR-PATH/stellarpath-cli.git`
- `cd⠀stellarpath-cli`
- `cargo⠀build⠀--release`
- `sudo⠀cp⠀target/release/stellarpath⠀/usr/local/bin/`


### Method C: Docker Image

- `docker⠀pull⠀ghcr.io/stellar-path/stellarpath-cli:latest`
- `docker⠀run⠀-v⠀$(pwd):/workspace⠀ghcr.io/stellar-path/stellarpath-cli⠀scan⠀/workspace`


---

## 4. CLI Command Matrix

| Command | Flags / Arguments | Description | Output Format |
| :--- | :--- | :--- | :--- |
| `scan` | `<DIR> --format [term,json,sarif]` | Executes the primary AST analysis traversal over all `.rs` files in the target directory. | Terminal, JSON, SARIF |
| `tree` | `<DIR> --depth <N>` | Outputs a structural map of the repository, identifying contract entrypoints and dependencies. | ASCII Tree |
| `doctor` | `--fix` | Analyzes `Cargo.toml` and `.cargo/config.toml` to verify Soroban SDK pinning and optimization flags. | Terminal |
| `explain`| `[ERROR_CODE]` | Provides long-form, detailed explanations of specific security anti-patterns and remediation steps. | Markdown |

### Configuration: `stellarpath.toml`
Place a `stellarpath.toml` in your repository root to configure the engine:

- `[core]`
- `strict_mode⠀=⠀true⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀#⠀Fails⠀CI⠀on⠀ANY⠀warning`
- `exclude_dirs⠀=⠀["tests/",⠀"benches/"]`
- `⠀`
- `[rules]`
- `e0001_raw_datakey⠀=⠀"deny"`
- `e0002_missing_ttl⠀=⠀"warn"`
- `e0003_unwrap_used⠀=⠀"allow"`


---

## 5. Error Codes & Security Diagnostics

`stellarpath-cli` assigns unique diagnostic codes to every detected anti-pattern.

### `E0001`: Raw Symbol DataKey Collision
Using raw `Symbol::short("admin")` directly in storage operations can lead to unintended collisions in complex contracts.

**Vulnerable Code:**

- `env.storage().instance().set(&Symbol::short("admin"),⠀&admin_address);`


**Remediated Code:**

- `#[contracttype]`
- `#[derive(Clone)]`
- `pub⠀enum⠀DataKey⠀{`
- `⠀⠀⠀⠀Admin,`
- `⠀⠀⠀⠀Allowance(Address),`
- `}`
- `⠀`
- `env.storage().instance().set(&DataKey::Admin,⠀&admin_address);`


### `E0002`: Missing Storage TTL Extension
Soroban state requires TTL extensions. Writing state without subsequently extending its TTL is flagged as a high-severity risk.

**Vulnerable Code:**

- `env.storage().persistent().set(&DataKey::Balance,⠀&amount);`
- `//⠀Missing⠀extend_ttl⠀call`


### `E0003`: Insecure RPC/Horizon Overlap
Flagging legacy Horizon endpoints when modern Soroban RPC endpoints should be used for data indexing.

---

## 6. Performance & False Positive Mitigation

`stellarpath-cli` is designed for ultra-low latency execution, typically scanning complex workspaces in under 50 milliseconds. 

**Zero-Cost Traversal**: Because `stellarpath-cli` never compiles to WASM, it skips LLVM IR generation entirely. It leverages `rayon` to parse multiple source files in parallel, allowing it to scale linearly with your CPU core count.

**False Positive Mitigation via Type Inference**: Unlike standard regex grep tools, `stellarpath-cli` infers the context of methods. An `.unwrap()` call on a local standard library `Option` can be distinguished from an `.unwrap()` on a highly sensitive state return, reducing CI fatigue and ensuring only valid security threats are surfaced.

---

## 7. Deep Dive: AST Traversal Mechanics

The structural advantage of AST verification over standard linting is context preservation. When `stellarpath-cli` evaluates a smart contract:
1. It resolves macro expansions for `#[contractimpl]` to track precisely which functions are public entrypoints.
2. It constructs a call graph indicating which internal helper functions mutate `env.storage()`.
3. It validates that instances where the Soroban `Env` is passed down the call chain retain the necessary authorization and TTL security checks.

This level of depth is what makes `stellarpath-cli` a true deterministic security engine, rather than just a code formatting tool.

---
<div align="center">
  <sub>Part of the <b>STELLAR-PATH</b> Toolchain. Built for the Soroban ecosystem.</sub>
</div>
