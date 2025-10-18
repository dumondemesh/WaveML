#!/usr/bin/env python3
import argparse, subprocess, json, time, os, sys
from concurrent.futures import ThreadPoolExecutor, as_completed

def run_one(wavectl, path):
    try:
        out = subprocess.check_output([wavectl, "forge", "--input", path, "--print-id"], text=True)
        idhex = out.strip().splitlines()[0].strip()
        return {"path": path, "id": idhex}
    except Exception as e:
        return {"path": path, "error": str(e)}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--wavectl", required=True)
    ap.add_argument("--list", required=True, help="file with newline-separated paths")
    ap.add_argument("--out", required=True)
    ap.add_argument("--jobs", type=int, default=1)
    args = ap.parse_args()

    with open(args.list, "r", encoding="utf-8") as f:
        items = [line.strip() for line in f if line.strip()]

    t0 = time.time()
    if args.jobs <= 1:
        results = [run_one(args.wavectl, p) for p in items]
    else:
        results = []
        with ThreadPoolExecutor(max_workers=args.jobs) as ex:
            futs = {ex.submit(run_one, args.wavectl, p): p for p in items}
            for fut in as_completed(futs):
                results.append(fut.result())
    t1 = time.time()

    # Sort deterministically by path then id
    results.sort(key=lambda x: (x.get("path",""), x.get("id","")))

    # Write as lines OR JSON depending on extension
    outp = args.out
    if outp.endswith(".json"):
        with open(outp, "w", encoding="utf-8") as f:
            json.dump({"elapsed_sec": round(t1 - t0, 6), "items": results}, f, ensure_ascii=False, indent=2)
    else:
        with open(outp, "w", encoding="utf-8") as f:
            for r in results:
                if "id" in r:
                    f.write(f"{r['id']} {r['path']}\n")
                else:
                    f.write(f"ERROR {r['path']} {r.get('error','')}\n")

if __name__ == "__main__":
    main()
