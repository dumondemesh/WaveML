#!/usr/bin/env python3
import sys, json, argparse, pathlib, numbers

def ensure_obj(parent, key):
    if key in parent and parent[key] is None:
        parent[key] = {}
    if key not in parent or not isinstance(parent[key], dict):
        parent[key] = {}
    return parent[key]

def set_default(obj, key, value):
    if key not in obj or obj[key] is None:
        obj[key] = value

def normalize_w_perf(w_perf):
    # threads -> int
    th = w_perf.get("threads", 0)
    if th is None:
        th = 0
    try:
        th = int(th)
    except Exception:
        th = 0
    w_perf["threads"] = th

    # frames -> int
    fr = w_perf.get("frames", 0)
    if fr is None:
        fr = 0
    try:
        fr = int(fr)
    except Exception:
        fr = 0
    w_perf["frames"] = fr

    # wall_ms -> float
    wm = w_perf.get("wall_ms", 0.0)
    if wm is None:
        wm = 0.0
    try:
        wm = float(wm)
    except Exception:
        wm = 0.0
    w_perf["wall_ms"] = wm

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--output", required=True)
    args = ap.parse_args()

    with open(args.input, "r", encoding="utf-8") as f:
        data = json.load(f)

    # Header/meta guards
    if not isinstance(data.get("header"), dict):
        data["header"] = {"schema_semver": "1.0.0"}
    else:
        data["header"].setdefault("schema_semver", "1.0.0")

    if not isinstance(data.get("meta"), dict):
        data["meta"] = {"generated_at": None, "tool": "wavectl"}
    else:
        data["meta"].setdefault("generated_at", None)
        data["meta"].setdefault("tool", "wavectl")

    # MDL null -> {}
    ensure_obj(data, "mdl")

    # Root-level w_params / w_perf
    w_params = ensure_obj(data, "w_params")
    w_perf = ensure_obj(data, "w_perf")

    # Fill required fields in w_params if absent
    set_default(w_params, "n_fft", 0)
    set_default(w_params, "hop", 0)
    set_default(w_params, "window", "NA")
    set_default(w_params, "mode", "NA")
    if "pad_mode" not in w_params:
        w_params["pad_mode"] = w_params["mode"]

    # Normalize performance fields
    normalize_w_perf(w_perf)

    # Nested W: { params, perf } null -> {}
    W = data.get("W")
    if isinstance(W, dict):
        W_params = ensure_obj(W, "params")
        W_perf = ensure_obj(W, "perf")
        # Optionally normalize nested perf as well
        normalize_w_perf(W_perf)
        data["W"] = W

    # Write output
    out = pathlib.Path(args.output)
    out.parent.mkdir(parents=True, exist_ok=True)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print("[SANITIZE] OK", file=sys.stderr)

if __name__ == "__main__":
    main()
