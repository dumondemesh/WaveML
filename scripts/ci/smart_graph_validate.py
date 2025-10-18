#!/usr/bin/env python3
import sys, json, argparse, os
from jsonschema import Draft7Validator

def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def validate_with_schema(schema_path, target):
    try:
        schema = load_json(schema_path)
    except Exception as e:
        return False, [f"[SCHEMA] cannot load schema {schema_path}: {e}"]
    v = Draft7Validator(schema)
    errors = sorted(v.iter_errors(target), key=lambda e: e.path)
    if errors:
        msgs = []
        for e in errors[:8]:
            loc = ".".join([str(x) for x in e.path])
            msgs.append(f"[SCHEMA] {loc or '<root>'}: {e.message}")
        msgs.append(f"[SCHEMA] total errors: {len(errors)}")
        return False, msgs
    return True, []

def compat_legacy_graph(target):
    """
    Legacy 'graph' object with nodes/edges, nodes carry 'id' and string 'op'.
    """
    if not isinstance(target, dict):
        return False, "compat-legacy: target is not an object"
    g = target.get("graph")
    if not isinstance(g, dict):
        return False, "compat-legacy: 'graph' not an object"
    nodes = g.get("nodes")
    edges = g.get("edges", [])
    if not isinstance(nodes, list):
        return False, "compat-legacy: 'nodes' not a list"
    if not isinstance(edges, list):
        return False, "compat-legacy: 'edges' not a list"
    for i, n in enumerate(nodes):
        if not isinstance(n, dict):
            return False, f"compat-legacy: node[{i}] not object"
        if "id" not in n or "op" not in n:
            return False, f"compat-legacy: node[{i}] missing 'id' or 'op'"
        if not isinstance(n["op"], str):
            return False, f"compat-legacy: node[{i}].op must be string"
    return True, "compat-legacy OK"

def compat_nf_shape(target):
    """
    NF-shaped document: 'graph' object with 'nodes' list whose entries may lack 'id',
    but must have string 'op' and optional 'params'. 'edges' optional.
    """
    if not isinstance(target, dict):
        return False, "compat-nf: target is not an object"
    g = target.get("graph")
    if not isinstance(g, dict):
        return False, "compat-nf: 'graph' not an object"
    nodes = g.get("nodes")
    if not isinstance(nodes, list):
        return False, "compat-nf: 'nodes' not a list"
    for i, n in enumerate(nodes):
        if not isinstance(n, dict):
            return False, f"compat-nf: node[{i}] not object"
        if "op" not in n or not isinstance(n["op"], str):
            return False, f"compat-nf: node[{i}].op missing or not string"
        if "params" in n and not isinstance(n["params"], dict):
            return False, f"compat-nf: node[{i}].params must be object if present"
    edges = g.get("edges", [])
    if "edges" in g and not isinstance(edges, list):
        return False, "compat-nf: 'edges' must be list if present"
    return True, "compat-nf OK"

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--primary", required=False)
    ap.add_argument("--alt", required=False)
    ap.add_argument("--target", required=True)
    args = ap.parse_args()

    target = load_json(args.target)

    if args.primary and os.path.exists(args.primary):
        ok, msgs = validate_with_schema(args.primary, target)
        if ok:
            print("[SCHEMA] OK (primary)")
            return 0
        else:
            for m in msgs: print(m, file=sys.stderr)

    if args.alt and os.path.exists(args.alt):
        ok, msgs = validate_with_schema(args.alt, target)
        if ok:
            print("[SCHEMA] OK (alt)")
            return 0
        else:
            for m in msgs: print(m, file=sys.stderr)

    ok_legacy, msg_legacy = compat_legacy_graph(target)
    if ok_legacy:
        print("[SCHEMA] COMPAT OK — legacy 'graph{nodes,edges}' accepted. Consider migrating to WMLB-1.1.")
        return 0

    ok_nf, msg_nf = compat_nf_shape(target)
    if ok_nf:
        print("[SCHEMA] COMPAT OK — NF-shaped 'graph{nodes(op,params),edges?}' accepted. Define NF schema or migrate examples.")
        return 0

    print("[SCHEMA] FAIL — target does not match primary/alt schemas or compat (legacy/NF).", file=sys.stderr)
    return 1

if __name__ == "__main__":
    sys.exit(main())
