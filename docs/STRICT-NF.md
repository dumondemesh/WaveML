# STRICT-NF (Canonical Normal Form)

**Ключ сортировки (обновлён):**  
`(op, n_fft, hop, window, center, pad_mode)`

- `op` ∈ Σ = {T, D, W, A, C, Φ, Align}
- `n_fft`, `hop` — целые параметры для оконных преобразований.
- `window` — {Hann, Hamming, Blackman, ...}
- `center` — булев флаг центрирования окна (влияет на causality/latency).
- `pad_mode` — {reflect, toeplitz}; **zero-pad запрещён (R7)**.

## Инварианты
- **I1**: единая STRICT-NF для эквивалентных графов.
- **I2/I3**: монотонность и консервативность при допустимых свопах.
- **I4/I5**: десцент и MDL-консистентность (будет обеспечено в WaveTrain).

## Примеры
Эквивалентные графы (разные представления) → одинаковый `NF-ID`.  
Различающиеся `center` или `pad_mode` → **разные** `NF-ID`.

См. `ci/forge_gate.sh` и `examples/graph/forge_diff_*.json`.
