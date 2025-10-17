# Sprint 2 – WaveRunner v0 + Phase-Lock (scaffold)

**Goals**
1. Mini-runtime for IR (execute simple chains): `W -> A -> C`.
2. Phase-Lock skeleton (Φ) with coherence penalty wired into WFR.
3. Audio dialect preset (periodic Hann defaults, amp/power modes).

**Deliverables**
- `crates/waverunner` (toy executor, streaming tick API).
- Extend `wavectl simulate-swaps` with Φ-aware swaps & ΔL_coh.
- Acceptance: Φ-swap scenarios; add `cert.i2/i3` examples with `λ·L_coh` effects.
- Update schema examples, docs, CI.

**Stretch**
- WaveTrain v0 sketch for Sprint 3 (SGD/Adam on toy graphs) and I4/I5 probes.
