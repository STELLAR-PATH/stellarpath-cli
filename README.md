# StellarPath

CLI for safely navigating and analyzing Stellar/Soroban repositories.

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Scan current directory
stellar_path scan .

# Scan and output JSON
stellar_path scan . --format json

# Get starting recommendations
stellar_path start .
```
