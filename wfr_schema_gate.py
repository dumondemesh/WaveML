#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
WFR Schema Gate v2
- Enforces schema_version=1.0.0
- Requires 'cert' and cert.i1..i5 to be present (values may be bool or None)
- If w_params.n_fft/hop exist -> require w_perf.cola_pass (bool) & w_perf.cola_rel_dev (number)
Exit code: 0 on OK, 1 on FAIL.
"""
import sys, os, json
from typing import Any, Dict, List

REQ_CERT_KEYS = ["i1", "i2", "i3", "i4", "i5"]

def is_number(x: Any) -> bool:
    return isinstance(x, (int, float)) and not isinstance(x, bool)

def check_file(path: str) -> List[str]:
    errs: List[str] = []
    try:
        with open(path, "r", encoding="utf-8") as f:
            obj = json.load(f)
    except Exception as e:
        errs.append(f"{path}: json load error: {e}")
        return errs

    # 1) schema_version
    schema = obj.get("schema_version")
    if schema != "1.0.0":
        errs.append(f"{path}: schema_version must be '1.0.0' (got {schema!r})")

    # 2) cert presence + keys (bool|None)
    cert = obj.get("cert")
    if cert is None or not isinstance(cert, dict):
        errs.append(f"{path}: missing key: cert")
    else:
        for k in REQ_CERT_KEYS:
            if k not in cert:
                errs.append(f"{path}: cert.{k} missing")
            else:
                v = cert.get(k)
                if v is not None and not isinstance(v, bool):
                    errs.append(f"{path}: cert.{k} must be bool or null (got {type(v).__name__})")

    # 3) If w_params has n_fft & hop -> require w_perf keys
    wp = obj.get("w_params") or {}
    has_wp = isinstance(wp, dict) and ("n_fft" in wp and "hop" in wp and wp.get("n_fft") is not None and wp.get("hop") is not None)
    if has_wp:
        wperf = obj.get("w_perf")
        if wperf is None or not isinstance(wperf, dict):
            errs.append(f"{path}: missing key: w_perf")
        else:
            if "cola_pass" not in wperf:
                errs.append(f"{path}: w_perf.cola_pass missing")
            else:
                cp = wperf.get("cola_pass")
                if not isinstance(cp, bool):
                    errs.append(f"{path}: w_perf.cola_pass must be bool (got {type(cp).__name__})")
            if "cola_rel_dev" not in wperf:
                errs.append(f"{path}: w_perf.cola_rel_dev missing")
            else:
                crd = wperf.get("cola_rel_dev")
                if not isinstance(crd, (int, float)):
                    errs.append(f"{path}: w_perf.cola_rel_dev must be number (got {type(crd).__name__})")

    return errs

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 ci/wfr_schema_gate.py <base_dir> [--quiet]", file=sys.stderr)
        sys.exit(2)
    base = sys.argv[1]
    quiet = "--quiet" in sys.argv[2:]

    issues: List[str] = []
    for root, _, files in os.walk(base):
        for fn in files:
            if not fn.endswith(".json"):
                continue
            if not fn.endswith(".wfr.json"):
                continue
            path = os.path.join(root, fn)
            errs = check_file(path)
            issues.extend(errs)

    if issues:
        if not quiet:
            print("SCHEMA-GATE: FAIL")
            for e in issues:
                print(" -", e)
        sys.exit(1)
    else:
        if not quiet:
            print("SCHEMA-GATE: OK")
        sys.exit(0)

if __name__ == "__main__":
    main()