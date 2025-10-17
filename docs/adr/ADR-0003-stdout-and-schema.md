# ADR-0003: Stdout contract & Input schema (Phase P1/I1)

**Status:** Accepted  
**Context:** `wavectl forge` is used in scripts/pipes; non-deterministic or verbose stdout breaks automation.
We also need stable input validation to guarantee canonicalization correctness.

**Decision:**
- `--print-id` prints **only hex NF-ID on stdout**. Human banners go to stderr or require `--with-banner`.
- Support `--input -` for stdin and glob patterns for batch runs.
- Provide `schemas/graph.schema.json` (schema_semver) and a `--schema` flag for validation (full validator can be added).

**Consequences:**
- Deterministic machine-readable output by default.
- Lower friction in CI and batch tooling.
- A formal contract for input structure (extensible per schema_semver).
