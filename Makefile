# Makefile — convenience targets for WaveML CI

.PHONY: all fast ci gates clean

all:
	cargo build
	cargo clippy --workspace --all-targets -- -D warnings
	bash scripts/ci/run_all_gates.sh

fast:
	cargo build
	bash scripts/ci/forge_gate.sh
	bash scripts/ci/schema_gate.sh
	bash scripts/ci/property_gate.sh

ci:
	bash scripts/ci/run_all_gates.sh

gates:
	bash scripts/ci/forge_gate.sh && \
	bash scripts/ci/schema_gate.sh && \
	bash scripts/ci/property_gate.sh && \
	bash scripts/ci/swaps_gate.sh && \
	bash scripts/ci/wt_equiv_gate.sh && \
	bash scripts/ci/perf_gate.sh

clean:
	rm -rf build/ci_logs build/acceptance_i2 build/acceptance_i3 build/perf
