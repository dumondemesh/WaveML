#!/usr/bin/env python3
import sys, json, argparse, yaml

def load_yaml(path):
    with open(path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f) or {}

def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--thr", required=True, help="thresholds.yaml")
    ap.add_argument("--files", nargs="+", required=True, help="WFR json files to check")
    args = ap.parse_args()

    cfg = load_yaml(args.thr)
    thr = cfg.get("i3", {})
    mse_max = float(thr.get("wt_mse_max", 1e-9))
    sdr_min = float(thr.get("sdr_db_min", 60.0))
    cola_max = float(thr.get("cola_max_dev", 1e-12))

    # New flags (default: optional in F4)
    require_sdr = bool(thr.get("require_sdr", False))
    require_cola = bool(thr.get("require_cola", False))

    ok = True
    for f in args.files:
        data = load_json(f)
        metrics = data.get("metrics", {}) if isinstance(data.get("metrics", {}), dict) else {}
        mse = metrics.get("mse")
        sdr = metrics.get("sdr_db")
        cola = metrics.get("cola_max_dev")

        # MSE is mandatory
        if mse is None:
            print(f"[FAIL] {f}: missing metric 'mse'", file=sys.stderr)
            ok = False
            continue

        # Check MSE threshold
        if mse > mse_max:
            print(f"[FAIL] {f}: mse={mse} > {mse_max}", file=sys.stderr)
            ok = False
            continue  # no need to check optional metrics if core failed

        # Optional metrics
        warn_msgs = []
        if require_sdr:
            if sdr is None:
                print(f"[FAIL] {f}: missing required 'sdr_db'", file=sys.stderr); ok = False
            elif sdr < sdr_min:
                print(f"[FAIL] {f}: sdr_db={sdr} < {sdr_min}", file=sys.stderr); ok = False
        else:
            if sdr is None:
                warn_msgs.append("sdr_db missing (optional)")

        if require_cola:
            if cola is None:
                print(f"[FAIL] {f}: missing required 'cola_max_dev'", file=sys.stderr); ok = False
            elif cola > cola_max:
                print(f"[FAIL] {f}: cola_max_dev={cola} > {cola_max}", file=sys.stderr); ok = False
        else:
            if cola is None:
                warn_msgs.append("cola_max_dev missing (optional)")

        if ok:
            if warn_msgs:
                print(f"[OK] {f}: mse={mse} — " + "; ".join(warn_msgs))
            else:
                print(f"[OK] {f}: mse={mse}, sdr_db={sdr}, cola_max_dev={cola}")

    if not ok:
        sys.exit(1)

if __name__ == "__main__":
    main()
