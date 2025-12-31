#!/bin/bash
# Run MMSB analyzer on this project

set -euo pipefail

ROOT_DIR="/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
OUTPUT_DIR="$ROOT_DIR/docs"

cd "$ROOT_DIR"

# Pick one (uncomment the block you want):
# 1) Full workflow: build + analyze + TODO report with severity exit codes
cargo run --manifest-path "$ROOT_DIR/xtask/Cargo.toml" -- \
    check \
    --root "$ROOT_DIR" \
    --output "$OUTPUT_DIR" \
    --skip-julia \
    --dead-code \
    --dead-code-filter \
    --dead-code-json "$OUTPUT_DIR/98_dead_code/dead_code_full.json" \
    --dead-code-summary "$OUTPUT_DIR/98_dead_code/dead_code_summary.md" \
    --dead-code-summary-limit 50 \
    --dead-code-policy "$OUTPUT_DIR/98_dead_code/dead_code_policy.txt" \
    --correction-intelligence \
    --correction-json "$OUTPUT_DIR/97_correction_intelligence/correction_intelligence.json" \
    --verification-policy-json "$OUTPUT_DIR/97_correction_intelligence/verification_policy.json" \
    --correction-path-slice \
    --correction-visibility-slice \
    "$@" | tee "$ROOT_DIR/report_check.txt"

# 2) Analyze only (regenerate docs, no TODO report)
# cargo run --manifest-path "$ROOT_DIR/xtask/Cargo.toml" -- \
#     analyze \
#     --root "$ROOT_DIR" \
#     --output "$OUTPUT_DIR" \
#     --skip-julia \
#     --dead-code \
#     --dead-code-filter \
#     --dead-code-json "$OUTPUT_DIR/98_dead_code/dead_code_full.json" \
#     --dead-code-summary "$OUTPUT_DIR/98_dead_code/dead_code_summary.md" \
#     --dead-code-summary-limit 50 \
#     --dead-code-policy "$OUTPUT_DIR/98_dead_code/dead_code_policy.txt" \
#     --correction-intelligence \
#     --correction-json "$OUTPUT_DIR/97_correction_intelligence/correction_intelligence.json" \
#     --verification-policy-json "$OUTPUT_DIR/97_correction_intelligence/verification_policy.json" \
#     --correction-path-slice \
#     --correction-visibility-slice \
#     "$@" | tee "$ROOT_DIR/report_analyze.txt"

# 3) Report only (parse existing docs, no re-analysis)
# cargo run --manifest-path "$ROOT_DIR/xtask/Cargo.toml" -- \
#     report \
#     --docs-dir "$OUTPUT_DIR" \
#     "$@" | tee "$ROOT_DIR/report_only.txt"
