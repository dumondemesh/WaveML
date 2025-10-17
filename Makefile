.PHONY: all build clippy cola validate acceptance ci

all: build clippy cola validate

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
