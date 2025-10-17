# CI pipeline — next iteration

Adds:
- Artifact upload (build_migrated & build_i23)
- JSON acceptance runner (`tools/acceptance_runner.py`)
- Keeps clippy(strict), tests, migration & metrics, and I2/I3 scaffold

Local one-liner:
  bash ci/run_wfr_migration_and_check.sh build build_migrated &&   bash ci/run_i23_acceptance.sh build_i23 &&   python3 tools/verify_i23.py build_i23 &&   python3 tools/acceptance_runner.py acceptance/tests_i2_i3.json
