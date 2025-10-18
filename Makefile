.PHONY: all fast clean acceptance release gates

all: build clippy acceptance gates

fast: build gates

build:
	cargo build

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

acceptance:
	cargo run -p wavectl -- acceptance --plan acceptance/tests.yaml --outdir build/acceptance --strict || true

gates:
	bash scripts/ci/run_all_gates.sh

clean:
	rm -rf target build out *.log **/*.log

release:
	cargo build --release
