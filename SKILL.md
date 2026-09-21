---
name: stellarpath-cli
description: "A fast, Rust-based static analyzer for Stellar and Soroban repositories."
---

# stellarpath-cli

`stellarpath-cli` is a fast, locally-run static analysis tool that identifies what a Stellar/Soroban codebase does and flags common security mistakes.

## What it does
- Detects usage of the Stellar SDK, distinguishing between modern RPC and legacy Horizon endpoints.
- Detects Soroban smart contracts, distinguishing structurally sound patterns (like `require_auth` and safe math) from common anti-patterns.
- Identifies specific Stellar Ecosystem Proposals (SEPs) implemented in the codebase (e.g., SEP-10, SEP-24, SEP-53).
- Provides a dedicated `lint` mode that flags a subset of the 23 documented Soroban common mistakes (e.g., missing events, unsafe unwraps, bare panics).

## How an AI agent should invoke it
You can run the tool in three modes:

1. **Scan Mode**: Outputs a summary of what the project is and what components it contains.
   ```bash
   cargo run -- scan . --format markdown
   ```
2. **Start Mode**: Provides concrete starting recommendations for exploring or updating the codebase.
   ```bash
   cargo run -- start .
   ```
3. **Security Lint Mode**: Scans specifically for smart contract vulnerabilities and anti-patterns.
   ```bash
   cargo run -- lint .
   ```

## What to expect
- **Scan/Start**: The output will be a structured report (in terminal, JSON, or Markdown) listing all discovered evidence with associated file paths, reasoning, and component types. 
- **Lint**: The output will specifically list "Security Lint Findings" pointing directly to the file, line, and specific mistake rule number (e.g., Mistake #17).

## What it should NOT be used for
- Do NOT use `stellarpath-cli` as a replacement for a professional security audit. 
- The `lint` mode does NOT cover all 23 common Soroban mistakes (it skips heuristics-heavy ones like reentrancy and complex TTL management).
- Do NOT use the `scan` mode to look for security issues, as the `scan` output is focused solely on identifying the components of the codebase, not its flaws.
