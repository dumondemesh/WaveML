# Demo W package (.wmpkg via .wmlb)

Pack to `.wmlb`:
```bash
# pack only manifest (no blobs) for the demo
./target/debug/wavectl pack \
  --manifest modules/demo_wpkg/manifest.json \
  --blob graph=modules/demo_wpkg/manifest.json \
  --out build/demo_wpkg.wmlb

# inspect
./target/debug/wavectl inspect --input build/demo_wpkg.wmlb

# report-from-graph directly from the .wmlb
./target/debug/wavectl report-from-graph \
  --input build/demo_wpkg.wmlb \
  --out build/reports/demo_wpkg.wfr.json \
  --mode amp --tol 1e-12
```
