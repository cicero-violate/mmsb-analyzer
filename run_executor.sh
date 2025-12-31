#!/bin/bash
# Run MMSB executor against correction intelligence output

set -euo pipefail

ROOT_DIR="/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-analyzer"
EXEC_DIR="/home/cicero-arch-omen/ai_sandbox/codex-agent/codex_sse/server-tools/MMSB/tools/mmsb-executor"
OUTPUT_DIR="$ROOT_DIR/docs/97_correction_intelligence/slice_path_coherence"
TIMEOUT_SECONDS=1200

cd "$EXEC_DIR"

# Pick one (uncomment the block you want):
# 1) Dry run (no mutations)
# timeout "$TIMEOUT_SECONDS" cargo run -- \
#     --root "$ROOT_DIR" \
#     --correction-json "$OUTPUT_DIR/correction_intelligence.json" \
#     --verification-policy-json "$OUTPUT_DIR/verification_policy.json" \
#     --report "$OUTPUT_DIR/executor_report.json" \
#     --verification-report "$OUTPUT_DIR/executor_verification_results.json" \
#     --rollback-log "$OUTPUT_DIR/executor_rollback_log.json" \
#     --diff-report "$OUTPUT_DIR/executor_diff_report.json" \
#     --input-report "$OUTPUT_DIR/executor_input_report.json" \
#     --print-diffs \
#     --diff-limit 5 \
#     --dry-run

# 2) Apply changes (mutations + verification)
# timeout "$TIMEOUT_SECONDS" cargo run -- \
#     --root "$ROOT_DIR" \
#     --correction-json "$OUTPUT_DIR/correction_intelligence.json" \
#     --verification-policy-json "$OUTPUT_DIR/verification_policy.json" \
#     --report "$OUTPUT_DIR/executor_report.json" \
#     --verification-report "$OUTPUT_DIR/executor_verification_results.json" \
#     --rollback-log "$OUTPUT_DIR/executor_rollback_log.json" \
#     --diff-report "$OUTPUT_DIR/executor_diff_report.json" \
#     --input-report "$OUTPUT_DIR/executor_input_report.json"
#     --print-diffs \
#     --diff-limit 5
#     --apply-path-coherence

# 3) Apply changes without verification
# timeout "$TIMEOUT_SECONDS" cargo run -- \
#     --root "$ROOT_DIR" \
#     --correction-json "$OUTPUT_DIR/correction_intelligence.json" \
#     --verification-policy-json "$OUTPUT_DIR/verification_policy.json" \
#     --report "$OUTPUT_DIR/executor_report.json" \
#     --verification-report "$OUTPUT_DIR/executor_verification_results.json" \
#     --rollback-log "$OUTPUT_DIR/executor_rollback_log.json" \
#     --diff-report "$OUTPUT_DIR/executor_diff_report.json" \
#     --input-report "$OUTPUT_DIR/executor_input_report.json" \
#     --print-diffs \
#     --diff-limit 5 \
#     --no-verify
#     --apply-path-coherence

# 4) MoveToLayer expansion (BundleMove allowlist, dry run)
OUTPUT_DIR="$ROOT_DIR/docs/97_correction_intelligence/slice_one_per_prefix"
timeout "$TIMEOUT_SECONDS" cargo run -- \
    --root "$ROOT_DIR" \
    --correction-json "$OUTPUT_DIR/correction_intelligence.json" \
    --verification-policy-json "$OUTPUT_DIR/verification_policy.json" \
    --report "$OUTPUT_DIR/executor_report.json" \
    --verification-report "$OUTPUT_DIR/executor_verification_results.json" \
    --rollback-log "$OUTPUT_DIR/executor_rollback_log.json" \
    --diff-report "$OUTPUT_DIR/executor_diff_report.json" \
    --input-report "$OUTPUT_DIR/executor_input_report.json" \
    --print-diffs \
    --diff-limit 5 \
    --apply-dependency-plan
