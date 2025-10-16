use anyhow::*;
use serde_json::Value;
use waveform::WaveForm;
use wmlb::Graph;

/// Выполнить граф над входным WaveForm.
pub fn run(g: &Graph, input: &WaveForm) -> Result<WaveForm> {
    let mut wf = input.clone();
    for n in &g.nodes {
        match n.op.as_str() {
            "W" => {
                // Пока заглушка: просто проверим поддерживаемые edge и пройдём дальше.
                if let Some(edge) = n.params.get("edge").and_then(|v| v.as_str()) {
                    if edge != "reflect" && edge != "Toeplitz" {
                        bail!("waverunner: unsupported edge='{edge}' for W");
                    }
                }
            }
            "D" => {
                let lambda = n
                    .params
                    .get("lambda")
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| anyhow!("waverunner: D requires numeric 'lambda'"))?;
                let aa = n
                    .params
                    .get("aa")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("waverunner: D requires 'aa' string"))?;

                wf = downsample_with_aa(&wf, lambda, aa)?;
            }
            "T" => {
                // NOP
            }
            other => bail!("waverunner: unknown op '{other}'"),
        }
    }
    Ok(wf)
}

/// Даунсемплинг по всем трекам с простым AA-FIR (sinc * Hann), отражающие границы.
fn downsample_with_aa(wf: &WaveForm, lambda: f64, aa: &str) -> Result<WaveForm> {
    if lambda <= 1.0 {
        bail!("waverunner: lambda must be > 1");
    }
    if aa != "sinc" {
        bail!("waverunner: only aa=\"sinc\" is supported for now");
    }

    // читаем mono-трек из wf.tracks as JSON: {"mono":[...]}
    let (mono, other_keys) = extract_mono(&wf.tracks)?;

    // FIR-параметры
    let lmb = lambda;
    let cutoff = 0.5 / lmb; // НЧ-срез до частоты Найквиста после D
    let taps = 31; // нечётное число
    let h = make_lowpass_sinc_hann(cutoff, taps);

    // Фильтрация с отражением на границах
    let filtered = conv_reflect(&mono, &h);

    // Даунсемплинг: берём каждый λ-й отсчёт
    let step = lmb.round() as usize;
    let mut out = Vec::with_capacity((filtered.len() + step - 1) / step);
    let mut idx = 0usize;
    while idx < filtered.len() {
        out.push(filtered[idx]);
        idx += step;
    }

    // Обновим частоту дискретизации (если была)
    let mut hdr = wf.header.clone();
    if let Some(rate) = hdr.rate {
        let new_rate = ((rate as f64) / lmb).round() as u32;
        hdr.rate = Some(new_rate.max(1));
    }

    // Соберём обновлённые tracks обратно в JSON
    let mut tracks = serde_json::Map::new();
    tracks.insert(
        "mono".into(),
        Value::Array(out.iter().map(|x| Value::from(*x)).collect()),
    );

    // Сохраним прочие ключи, если они были (кроме "mono")
    for (k, v) in other_keys {
        tracks.insert(k, v);
    }

    Ok(WaveForm {
        header: hdr,
        tracks: Value::Object(tracks),
        passports: wf.passports.clone(),
    })
}

/// Извлекает массив mono из JSON-объекта tracks и возвращает его вместе с остальными ключами.
fn extract_mono(tracks: &Value) -> Result<(Vec<f64>, Vec<(String, Value)>)> {
    let obj = tracks
        .as_object()
        .ok_or_else(|| anyhow!("waverunner: tracks must be an object like {{\"mono\":[..]}}"))?;

    let mono_v = obj
        .get("mono")
        .ok_or_else(|| anyhow!("waverunner: tracks.mono not found"))?;

    let mono_arr = mono_v
        .as_array()
        .ok_or_else(|| anyhow!("waverunner: tracks.mono must be an array"))?;

    let mut mono = Vec::with_capacity(mono_arr.len());
    for v in mono_arr {
        mono.push(
            v.as_f64()
                .ok_or_else(|| anyhow!("waverunner: tracks.mono must be numeric"))?,
        );
    }

    // Соберём другие ключи, чтобы не потерять пользовательские треки/данные
    let mut others = Vec::new();
    for (k, v) in obj.iter() {
        if k != "mono" {
            others.push((k.clone(), v.clone()));
        }
    }

    Ok((mono, others))
}

/// sinc lowpass с окном Хэннинга (Hann)
fn make_lowpass_sinc_hann(fc: f64, taps: usize) -> Vec<f64> {
    assert!(taps % 2 == 1, "taps must be odd");
    let m = taps as i64;
    let mid = m / 2;
    let mut h = Vec::with_capacity(taps);

    for n in 0..m {
        let k = n - mid;
        let x = k as f64;

        // sinc(2*pi*fc*x) с нормировкой
        let sinc = if x == 0.0 {
            2.0 * fc
        } else {
            (2.0 * std::f64::consts::PI * fc * x).sin() / (std::f64::consts::PI * x)
        };

        // окно Хэннинга
        let w = 0.5 * (1.0 - (2.0 * std::f64::consts::PI * (n as f64) / (m as f64 - 1.0)).cos());

        h.push(sinc * w);
    }

    // нормируем энергию (сумму коэффициентов) к 1.0
    let sum: f64 = h.iter().sum();
    if sum.abs() > 1e-12 {
        for v in &mut h {
            *v /= sum;
        }
    }
    h
}

/// Свертка с отражающими границами: y[n] = sum_k x[n-k]*h[k], k=0..L-1
fn conv_reflect(x: &[f64], h: &[f64]) -> Vec<f64> {
    let n = x.len();
    let l = h.len();
    if n == 0 {
        return vec![];
    }
    let mut y = vec![0.0; n];

    for n_idx in 0..n {
        let mut acc = 0.0;
        for k in 0..l {
            let tap = h[k];
            let x_idx = n_idx as isize - k as isize;
            let xx = sample_reflect(x, x_idx);
            acc += xx * tap;
        }
        y[n_idx] = acc;
    }
    y
}

/// Отражающий доступ: ..., 2,1,0,1,2,3,4,3,2,1,0,1,2, ...
fn sample_reflect(x: &[f64], idx: isize) -> f64 {
    let n = x.len() as isize;
    if n == 1 {
        return x[0];
    }
    // приводим idx в диапазон [0, n-1] c отражением
    let mut i = idx;
    if i < 0 {
        i = -i;
    }
    let period = 2 * (n - 1);
    i = i % period;
    if i >= n {
        i = period - i;
    }
    x[i as usize]
}
