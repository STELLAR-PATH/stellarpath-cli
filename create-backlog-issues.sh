#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-DANTE-1903/stellarpath-cli}"

if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "Error: GITHUB_TOKEN is not set."
  exit 1
fi

create_issue() {
  local title="$1"
  local body="$2"
  local labels="$3"

  echo "Creating issue: $title..."
  curl -s -X POST \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/"$REPO"/issues \
    -d "$(jq -n \
      --arg title "$title" \
      --arg body "$body" \
      --argjson labels "$labels" \
      '{title: $title, body: $body, labels: $labels}')" > /dev/null
}

echo "Seeding backlog issues for $REPO..."

create_issue \
  "feat(cli): add --color flag for CI and no-color terminal output" \
  "### Summary
Provide a flag to disable ANSI coloring for automation pipelines and raw log ingestion.

### Acceptance Criteria
- [ ] Add --color=always|auto|never flag in clap args
- [ ] Honor NO_COLOR environment variable
- [ ] Add unit test verifying stripped ANSI codes

### Tech Stack
Rust, Clap v4" \
  '["enhancement", "good first issue", "complexity: trivial"]'

create_issue \
  "feat(lint): add Soroban Security Mistake #1 reentrancy and arbitrary call checks" \
  "### Summary
Implement static analysis check detecting untrusted contract invocation patterns prior to state changes.

### Acceptance Criteria
- [ ] Implement AST visitor inspecting cross-contract call ordering
- [ ] Add diagnostic reporting file path, line number, and remediation guidance
- [ ] Provide passing and failing Soroban test contracts under testdata/

### Tech Stack
Rust, Syn AST, Soroban SDK" \
  '["enhancement", "security", "complexity: medium"]'

create_issue \
  "feat(indexer): integrate Soroban RPC event subscription and live health watcher" \
  "### Summary
Add real-time diagnostic polling against modern Soroban RPC endpoints to detect active ledger contracts missing event emissions.

### Acceptance Criteria
- [ ] Build RPC event listener polling getEvents method
- [ ] Format live diagnostic stream into terminal table
- [ ] Support custom RPC endpoint flag (--rpc-url)

### Tech Stack
Rust, Tokio, Reqwest, Soroban-RPC" \
  '["feature", "complexity: high"]'

echo "Backlog seeded successfully!"
