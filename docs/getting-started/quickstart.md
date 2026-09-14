# Quickstart
Run your first scan against any local Stellar or Soroban workspace in under a minute.
### 1. Run a Scan
Navigate to your Soroban contract directory and execute:
`stellarpath scan .`
### 2. Export Markdown Documentation
Generate documentation for contract security audits:
`stellarpath scan . --format markdown > ARCHITECTURE.md`
### 3. Machine-Readable CI Output
Export raw JSON for pipelines and lint bots:
`stellarpath scan . --format json > report.json`
