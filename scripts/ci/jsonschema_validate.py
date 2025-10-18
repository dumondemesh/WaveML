#!/usr/bin/env python3
import sys, json
from jsonschema import validate, Draft7Validator

if len(sys.argv) < 3:
    print("Usage: jsonschema_validate.py <schema.json> <target.json>", file=sys.stderr)
    sys.exit(2)

schema_path, target_path = sys.argv[1], sys.argv[2]
with open(schema_path, "r", encoding="utf-8") as f:
    schema = json.load(f)
with open(target_path, "r", encoding="utf-8") as f:
    target = json.load(f)

validator = Draft7Validator(schema)
errors = sorted(validator.iter_errors(target), key=lambda e: e.path)
if errors:
    for e in errors[:8]:
        loc = ".".join([str(x) for x in e.path])
        print(f"[SCHEMA] {target_path}:{loc}: {e.message}", file=sys.stderr)
    print(f"[SCHEMA] total errors: {len(errors)}", file=sys.stderr)
    sys.exit(1)
print("[SCHEMA] OK")
