# migrate_wfr.py — v1.2 hotfix

- Fix: crash on null/absent `w_params`.
- Ensures `w_perf` is always a dict and seeds backend/n_fft/hop when possible.
- Boolean parsing for `--default-center`.
