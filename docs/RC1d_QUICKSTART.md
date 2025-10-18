# RC1d — SelfContained CI

## Steps
```bash
unzip -o WaveML_F5_RC1d_SelfContained.zip -d .
chmod +x scripts/ci/* tools/* scripts/repair/* || true
bash scripts/ci/run_all_gates_rc1d.sh
```

If cargo fails on missing crates:
```bash
bash scripts/repair/trim_workspace.sh
bash scripts/ci/run_all_gates_rc1d.sh
```
