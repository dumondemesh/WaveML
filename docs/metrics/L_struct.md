# Структурная метрика L_struct (F2)

## Интуиция
`L_struct` — детерминированная сложность канона графа STRICT-NF:
суммируем вклад узлов с весами по типу и параметрам (штрафуем «дорогие» конфигурации),
без численного исполнения DSP.

## Определение (v1)
Для узла `W(n_fft, hop, window, center, pad_mode)`:

```
w_op = 1.0
w_fft = log2(n_fft).max(1.0)
w_hop = (n_fft as f64 / hop as f64).ln().max(0.0) * 0.25
w_win = {
  Hann: 0.0, Hamming: 0.05, Blackman: 0.075, Rect: 0.125, _other: 0.1
}
w_center = if center { 0.02 } else { 0.0 }
w_pad = { reflect: 0.0, toeplitz: 0.01, _other: 0.05 }

cost_W = w_op + w_fft + w_hop + w_win + w_center + w_pad
```

Итог: `L_struct = Σ cost(node) + λ_topo * (#edges)`; по умолчанию `λ_topo = 0.01`.

## Свойства
- Инвариантна к переименованию id узлов.
- Канонически сериализуется: одинаковые графы → одинаковый `L_struct`.
- При допустимых свопах (коммутирующие преобразования) — `ΔL_struct ≤ 0`.

## Настройка
Пороговая проверка `ΔL_struct ≤ 0` задаётся в `acceptance/thresholds.yaml`:
`i2.delta_l_struct_max: 1.0e-12` (допустимая погрешность 0 или ε).

## Отчётность (.wfr)
Пишется в `wfr.mdl.L_struct` и в `cert.i2_delta_l_le_0`.
