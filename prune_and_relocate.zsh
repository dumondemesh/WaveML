#!/usr/bin/env zsh
set -euo pipefail

APPLY=0
if [[ "${1:-}" == "--apply" ]]; then APPLY=1; fi
echo "== WaveML Prune Plan (20251018_150623) =="

echo "-- Planned moves:"
echo "move: CHANGELOG.md -> docs/CHANGELOG.md"
if [[ $APPLY -eq 1 ]]; then mkdir -p $(dirname 'docs/CHANGELOG.md'); git mv 'CHANGELOG.md' 'docs/CHANGELOG.md' || mv 'CHANGELOG.md' 'docs/CHANGELOG.md' || true; fi
echo "move: CHECKLIST.md -> docs/CHECKLIST_RC.md"
if [[ $APPLY -eq 1 ]]; then mkdir -p $(dirname 'docs/CHECKLIST_RC.md'); git mv 'CHECKLIST.md' 'docs/CHECKLIST_RC.md' || mv 'CHECKLIST.md' 'docs/CHECKLIST_RC.md' || true; fi
echo "move: versions.md -> docs/versions.md"
if [[ $APPLY -eq 1 ]]; then mkdir -p $(dirname 'docs/versions.md'); git mv 'versions.md' 'docs/versions.md' || mv 'versions.md' 'docs/versions.md' || true; fi
echo "-- Planned deletions:"
echo "delete: __MACOSX/ci/._run_all_gates.sh.bak.20251017231017 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/ci/._run_all_gates.sh.bak.20251017231017' 2>/dev/null || rm -rf '__MACOSX/ci/._run_all_gates.sh.bak.20251017231017'; fi
echo "delete: __MACOSX/ci/._run_all_gates.sh.force.bak.20251017231828 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/ci/._run_all_gates.sh.force.bak.20251017231828' 2>/dev/null || rm -rf '__MACOSX/ci/._run_all_gates.sh.force.bak.20251017231828'; fi
echo "delete: __MACOSX/crates/linters/src/._lib.rs.bak.linters_bool_fix (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/linters/src/._lib.rs.bak.linters_bool_fix' 2>/dev/null || rm -rf '__MACOSX/crates/linters/src/._lib.rs.bak.linters_bool_fix'; fi
echo "delete: __MACOSX/crates/wavectl/._Cargo.toml.bak.20251018_023913 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/._Cargo.toml.bak.20251018_023913' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/._Cargo.toml.bak.20251018_023913'; fi
echo "delete: __MACOSX/crates/wavectl/src/._cmd_nf_batch.rs.broken.bak (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/src/._cmd_nf_batch.rs.broken.bak' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/src/._cmd_nf_batch.rs.broken.bak'; fi
echo "delete: __MACOSX/crates/wavectl/src/._main.rs.bak.20251018_023116 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_023116' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_023116'; fi
echo "delete: __MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024131 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024131' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024131'; fi
echo "delete: __MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024408 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024408' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024408'; fi
echo "delete: __MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024950 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024950' 2>/dev/null || rm -rf '__MACOSX/crates/wavectl/src/._main.rs.bak.20251018_024950'; fi
echo "delete: __MACOSX/crates/wmlb/src/._lib.rs.bak (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/crates/wmlb/src/._lib.rs.bak' 2>/dev/null || rm -rf '__MACOSX/crates/wmlb/src/._lib.rs.bak'; fi
echo "delete: __MACOSX/scripts/ci/._run_all_gates.sh.bak.20251017231017 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/ci/._run_all_gates.sh.bak.20251017231017' 2>/dev/null || rm -rf '__MACOSX/scripts/ci/._run_all_gates.sh.bak.20251017231017'; fi
echo "delete: __MACOSX/scripts/ci/._run_all_gates.sh.force.bak.20251017231828 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/ci/._run_all_gates.sh.force.bak.20251017231828' 2>/dev/null || rm -rf '__MACOSX/scripts/ci/._run_all_gates.sh.force.bak.20251017231828'; fi
echo "delete: ci/run_all_gates.sh.bak.20251017231017 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'ci/run_all_gates.sh.bak.20251017231017' 2>/dev/null || rm -rf 'ci/run_all_gates.sh.bak.20251017231017'; fi
echo "delete: ci/run_all_gates.sh.force.bak.20251017231828 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'ci/run_all_gates.sh.force.bak.20251017231828' 2>/dev/null || rm -rf 'ci/run_all_gates.sh.force.bak.20251017231828'; fi
echo "delete: crates/linters/src/lib.rs.bak.linters_bool_fix (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/linters/src/lib.rs.bak.linters_bool_fix' 2>/dev/null || rm -rf 'crates/linters/src/lib.rs.bak.linters_bool_fix'; fi
echo "delete: crates/wavectl/Cargo.toml.bak.20251018_023913 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/Cargo.toml.bak.20251018_023913' 2>/dev/null || rm -rf 'crates/wavectl/Cargo.toml.bak.20251018_023913'; fi
echo "delete: crates/wavectl/src/cmd_cola.rs.bak_threads_fix (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/cmd_cola.rs.bak_threads_fix' 2>/dev/null || rm -rf 'crates/wavectl/src/cmd_cola.rs.bak_threads_fix'; fi
echo "delete: crates/wavectl/src/cmd_nf_batch.rs.broken.bak (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/cmd_nf_batch.rs.broken.bak' 2>/dev/null || rm -rf 'crates/wavectl/src/cmd_nf_batch.rs.broken.bak'; fi
echo "delete: crates/wavectl/src/cmd_report_from_graph.rs.bak_threads_fix (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/cmd_report_from_graph.rs.bak_threads_fix' 2>/dev/null || rm -rf 'crates/wavectl/src/cmd_report_from_graph.rs.bak_threads_fix'; fi
echo "delete: crates/wavectl/src/cmd_simulate_swaps.rs.bak_threads_fix (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/cmd_simulate_swaps.rs.bak_threads_fix' 2>/dev/null || rm -rf 'crates/wavectl/src/cmd_simulate_swaps.rs.bak_threads_fix'; fi
echo "delete: crates/wavectl/src/main.rs.bak.20251018_023116 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/main.rs.bak.20251018_023116' 2>/dev/null || rm -rf 'crates/wavectl/src/main.rs.bak.20251018_023116'; fi
echo "delete: crates/wavectl/src/main.rs.bak.20251018_024131 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/main.rs.bak.20251018_024131' 2>/dev/null || rm -rf 'crates/wavectl/src/main.rs.bak.20251018_024131'; fi
echo "delete: crates/wavectl/src/main.rs.bak.20251018_024408 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/main.rs.bak.20251018_024408' 2>/dev/null || rm -rf 'crates/wavectl/src/main.rs.bak.20251018_024408'; fi
echo "delete: crates/wavectl/src/main.rs.bak.20251018_024950 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wavectl/src/main.rs.bak.20251018_024950' 2>/dev/null || rm -rf 'crates/wavectl/src/main.rs.bak.20251018_024950'; fi
echo "delete: crates/wmlb/src/lib.rs.bak (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'crates/wmlb/src/lib.rs.bak' 2>/dev/null || rm -rf 'crates/wmlb/src/lib.rs.bak'; fi
echo "delete: scripts/ci/run_all_gates.sh.bak.20251017231017 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/ci/run_all_gates.sh.bak.20251017231017' 2>/dev/null || rm -rf 'scripts/ci/run_all_gates.sh.bak.20251017231017'; fi
echo "delete: scripts/ci/run_all_gates.sh.force.bak.20251017231828 (backup)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/ci/run_all_gates.sh.force.bak.20251017231828' 2>/dev/null || rm -rf 'scripts/ci/run_all_gates.sh.force.bak.20251017231828'; fi
echo "delete: .git/logs/refs/heads/release/v0.2.0-rc1 (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '.git/logs/refs/heads/release/v0.2.0-rc1' 2>/dev/null || rm -rf '.git/logs/refs/heads/release/v0.2.0-rc1'; fi
echo "delete: .git/refs/heads/release/v0.2.0-rc1 (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '.git/refs/heads/release/v0.2.0-rc1' 2>/dev/null || rm -rf '.git/refs/heads/release/v0.2.0-rc1'; fi
echo "delete: __MACOSX/docs/release/._F2_I2_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/docs/release/._F2_I2_DONE.md' 2>/dev/null || rm -rf '__MACOSX/docs/release/._F2_I2_DONE.md'; fi
echo "delete: __MACOSX/docs/release/._F3_I3_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/docs/release/._F3_I3_DONE.md' 2>/dev/null || rm -rf '__MACOSX/docs/release/._F3_I3_DONE.md'; fi
echo "delete: __MACOSX/docs/release/._P1_I1_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/docs/release/._P1_I1_DONE.md' 2>/dev/null || rm -rf '__MACOSX/docs/release/._P1_I1_DONE.md'; fi
echo "delete: __MACOSX/docs/release/._PHASE_STATUS.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/docs/release/._PHASE_STATUS.md' 2>/dev/null || rm -rf '__MACOSX/docs/release/._PHASE_STATUS.md'; fi
echo "delete: __MACOSX/scripts/release/._make_release.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/release/._make_release.sh' 2>/dev/null || rm -rf '__MACOSX/scripts/release/._make_release.sh'; fi
echo "delete: __MACOSX/scripts/release/._make_zip.zsh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/release/._make_zip.zsh' 2>/dev/null || rm -rf '__MACOSX/scripts/release/._make_zip.zsh'; fi
echo "delete: __MACOSX/scripts/release/._tag_f2_i2.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/release/._tag_f2_i2.sh' 2>/dev/null || rm -rf '__MACOSX/scripts/release/._tag_f2_i2.sh'; fi
echo "delete: __MACOSX/scripts/release/._tag_f3_i3.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/release/._tag_f3_i3.sh' 2>/dev/null || rm -rf '__MACOSX/scripts/release/._tag_f3_i3.sh'; fi
echo "delete: __MACOSX/scripts/release/._tag_p1_i1.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r '__MACOSX/scripts/release/._tag_p1_i1.sh' 2>/dev/null || rm -rf '__MACOSX/scripts/release/._tag_p1_i1.sh'; fi
echo "delete: docs/release/F2_I2_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'docs/release/F2_I2_DONE.md' 2>/dev/null || rm -rf 'docs/release/F2_I2_DONE.md'; fi
echo "delete: docs/release/F3_I3_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'docs/release/F3_I3_DONE.md' 2>/dev/null || rm -rf 'docs/release/F3_I3_DONE.md'; fi
echo "delete: docs/release/P1_I1_DONE.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'docs/release/P1_I1_DONE.md' 2>/dev/null || rm -rf 'docs/release/P1_I1_DONE.md'; fi
echo "delete: docs/release/PHASE_STATUS.md (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'docs/release/PHASE_STATUS.md' 2>/dev/null || rm -rf 'docs/release/PHASE_STATUS.md'; fi
echo "delete: scripts/release/make_release.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/release/make_release.sh' 2>/dev/null || rm -rf 'scripts/release/make_release.sh'; fi
echo "delete: scripts/release/make_zip.zsh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/release/make_zip.zsh' 2>/dev/null || rm -rf 'scripts/release/make_zip.zsh'; fi
echo "delete: scripts/release/tag_f2_i2.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/release/tag_f2_i2.sh' 2>/dev/null || rm -rf 'scripts/release/tag_f2_i2.sh'; fi
echo "delete: scripts/release/tag_f3_i3.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/release/tag_f3_i3.sh' 2>/dev/null || rm -rf 'scripts/release/tag_f3_i3.sh'; fi
echo "delete: scripts/release/tag_p1_i1.sh (generated-dir)"
if [[ $APPLY -eq 1 ]]; then git rm -r 'scripts/release/tag_p1_i1.sh' 2>/dev/null || rm -rf 'scripts/release/tag_p1_i1.sh'; fi
echo "== Done =="
