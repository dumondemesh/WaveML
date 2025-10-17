# Acceptance (Sprint-0 minimal)

## Quick start

```bash
# from this workspace root
cargo build

# pack a demo .wmlb
./target/debug/wavectl pack   --manifest examples/manifest_toy.json   --blob data=examples/blob_dummy.bin   --out examples/demo.wmlb

# inspect
./target/debug/wavectl inspect --input examples/demo.wmlb

# lint
./target/debug/wavectl lint --input examples/demo.wmlb

# report
./target/debug/wavectl report --module demo --out examples/demo.wfr.json
```

This overlay is additive: drop `crates/` subtree into your repo and add these crates as workspace members.
