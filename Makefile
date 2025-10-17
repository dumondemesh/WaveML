.PHONY: all build clippy cola validate acceptance ci

all: build clippy cola validate gates forge-gate nf-diff

build:
	cargo build

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

cola:
	target/debug/wavectl cola --n-fft 1024 --hop 512 --window Hann --mode amp --out build/reports/auto_amp.wfr.json

validate:
	target/debug/wavectl validate-wfr --wfr build/reports/auto_amp.wfr.json --require-pass

acceptance: ci

ci:
	bash scripts/ci/run_all_gates.sh

gates:       ## run all project gates
	@bash scripts/ci/run_all_gates.sh

forge-gate:  ## run only forge gate
	@bash ci/forge_gate.sh

nf-diff:     ## run only NF-DIFF gate
	@bash scripts/ci/nf_diff_gate.sh