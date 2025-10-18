# ADR-0004 — Graph schema compatibility
## Update — Smart Graph Validator (compat)

Во избежание ложных падений CI до завершения миграции, `schema_gate.sh` использует `smart_graph_validate.py`:
1) пробует `schemas/graph.schema.json` (primary),
2) затем `spec/WMLB-1.1.schema.json` (alt),
3) при провале — выполняет **compat-проверку**: корень содержит `graph`-объект с `nodes[]/edges[]`, а узлы имеют `id` и строковый `op`.

Если сработал compat-режим — печатается предупреждение: _«legacy 'graph{nodes,edges}' accepted. Consider migrating to WMLB-1.1.»_ и CI проходит. 
После миграции compat будет удалён, а primary станет указывать на спецификацию WMLB-1.1.
