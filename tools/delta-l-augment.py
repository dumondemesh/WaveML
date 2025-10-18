#!/usr/bin/env python3
import sys, json
def count_struct(d):
    if not isinstance(d, dict): return 0.0
    nodes=len(d.get("nodes",[]) or []); edges=len(d.get("edges",[]) or []); params=0
    for n in d.get("nodes",[]) or []:
        if isinstance(n,dict):
            p=n.get("params") or {}
            if isinstance(p,dict): params+=len(p.keys())
    return nodes + edges + 0.1*params
if __name__=="__main__":
    p=sys.argv[1]
    with open(p,"r",encoding="utf-8") as f: doc=json.load(f)
    before=doc.get("before") or {}; after=doc.get("after") or {}
    d=count_struct(after)-count_struct(before)
    doc.setdefault("mdl",{}).setdefault("i2",{})
    doc["mdl"]["i2"]["delta_l_struct"]=float(d)
    doc["mdl"]["i2"]["pass"]=(d<=0.0)
    with open(p,"w",encoding="utf-8") as f: json.dump(doc,f,ensure_ascii=False,indent=2)
    print(f"[OK] ΔL_struct={d}, pass={d<=0.0}")
