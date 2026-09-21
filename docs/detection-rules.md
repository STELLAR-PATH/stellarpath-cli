# Detection Rules

This document outlines exactly what code patterns trigger specific detectors in `stellarpath-cli`, enabling contributors to verify or extend detection without reverse-engineering the source.

## Stellar SDK / RPC vs Horizon
- **Stellar RPC Usage**: Triggers if a JS/TS/Go/Py/RS source file contains `rpc.Server` or `soroban.rpc`.
- **Horizon Usage (Legacy)**: Triggers if a source file contains `Horizon` or `horizon.stellar.org`. This is noted as a legacy path.

## Soroban Smart Contracts
- **Structurally Sound - Authorization**: Triggers if the contract code calls `require_auth`.
- **Structurally Sound - Storage Keys**: Triggers if the code contains `enum DataKey` or `#[contracttype]\npub enum`.
- **Structurally Sound - Safe Math**: Triggers if the code contains `checked_add`, `checked_sub`, or `checked_mul`.
- **Anti-pattern - Storage Key Collisions**: Triggers if raw symbols (like `symbol_short!(...)`) are used directly in `.set(...)`.

## Stellar Ecosystem Proposals (SEPs)
The SEP detector scans source files (`ts`, `js`, `go`, `py`, `rs`) for specific implementation patterns:

### SEP-10 (Stellar Web Authentication)
- **Triggers**: Code containing `buildChallengeTransaction`, `verifyChallengeTransaction`, `SEP-10`, or `SEP10`.

### SEP-24 (Hosted Deposit and Withdrawal)
- **Triggers**: Code containing `/transactions/deposit/interactive`, `/transactions/withdraw/interactive`, `SEP-24`, or `SEP24`.

### SEP-53 (Sign-In with Stellar)
- **Triggers**: Code containing `SEP-53`, `SEP53`, or `Sign-In with Stellar`.

## Security Lint Mode (`--security` / `lint`)
The security lint mode checks for a subset of the 23 known Soroban smart contract mistakes. 
Currently, the following are covered:
- **Mistake #17**: Bare `panic!` instead of typed errors.
- **Mistake #18**: Unsafe `unwrap()` / `expect()`.
- **Mistake #19**: Missing events.

The following are **NOT** covered by this static analysis tool (due to needing deeper heuristic analysis or AST parsing):
- #1: Missing `require_auth()`
- #2: Reinitialization
- #3: Wrong auth subject
- #4: Wrong storage type
- #5: Missing TTL extension
- #6: Storage key collisions
- #7: Temporary used for persistent data
- #8: Unchecked arithmetic
- #9: Division before multiplication / rounding
- #10: Missing input validation
- #11: Overwrite instead of accumulate
- #12: Missing state deduction
- #13: Arbitrary contract calls
- #14: Unvalidated cross-contract returns
- #15: Stellar Asset Contract assumptions
- #16: Frontrunning / slippage
- #20: Missing tests
- #21: Leaked secrets
- #22: Unpinned `soroban-sdk`
- #23: Unbounded loops
