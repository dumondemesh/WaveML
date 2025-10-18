#!/usr/bin/env python3
import argparse, json, os, subprocess, sys, time
from concurrent.futures import ThreadPoolExecutor, as_completed

def compute_id(wavectl, path):
    cmd = [wavectl, "forge", "--input", path, "--print-id"]
    out = subprocess.check_output(cmd, text=True).strip().splitlines()[0].strip()
    return path, out

def load_list(list_file, inputs):
    items = []
    if list_file:
        with open(list_file, "r") as f:
            for line in f:
                t = line.strip()
                if not t or t.startswith("#"): continue
                items.append(t)
    items.extend(inputs or [])
    return sorted(set(items))

def main():
    p = argparse.ArgumentParser(description="Deterministic batch NF-ID via wavectl (script-level, parallel).")
    p.add_argument("--wavectl", default="target/debug/wavectl")
    p.add_argument("--list")
    p.add_argument("--input", action="append", default=[])
    p.add_argument("--jobs", type=int, default=0)
    p.add_argument("--format", choices=["json","csv"], default="json")
    p.add_argument("--out")
    args = p.parse_args()

    items = load_list(args.list, args.input)
    if not items: print("[]"); return 0

    jobs = args.jobs
    if jobs <= 0:
        try:
            import multiprocessing as mp
            jobs = max(1, (mp.cpu_count() or 4) * 2)
        except Exception:
            jobs = 8

    results = []
    t0 = time.time()
    with ThreadPoolExecutor(max_workers=jobs) as ex:
        futs = { ex.submit(compute_id, args.wavectl, p): p for p in items }
        for fut in as_completed(futs):
            pth = futs[fut]
            try:
                ip, nf = fut.result()
                results.append({"input": ip, "nf_id": nf})
            except subprocess.CalledProcessError as e:
                sys.stderr.write(f"[nf-batch] ERROR on {pth}: {e}\n"); sys.exit(2)
    dt = time.time()-t0
    results.sort(key=lambda r: r["input"])

    if args.format=="json":
        payload = json.dumps(results, ensure_ascii=False, indent=2)
    else:
        lines=["input,nf_id"]; lines += [f'{r["input"]},{r["nf_id"]}' for r in results]
        payload = "\n".join(lines)

    if args.out:
        os.makedirs(os.path.dirname(args.out), exist_ok=True)
        open(args.out,"w").write(payload)
        print(f"[nf-batch] Wrote {args.out} ({len(results)} items, {dt:.3f}s)")
    else:
        print(payload)
    return 0

if __name__=="__main__":
    sys.exit(main())
