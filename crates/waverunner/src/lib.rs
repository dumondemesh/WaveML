pub mod ops;
use anyhow::*;
use serde_json::Value;
use wmlb::Graph;
use waveform::WaveForm;

/// Внутреннее состояние для пары W→T
struct StftBuf {
    frames: Vec<Vec<Complex>>,
    n_fft: usize,
    hop: usize,
    win: Vec<f64>,
    orig_len: usize,
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}
impl Complex {
    fn new(re: f64, im: f64) -> Self { Self { re, im } }
}

pub fn run(g: &Graph, input: &WaveForm) -> Result<WaveForm> {
    let mut wf = input.clone();
    // Храним STFT между W и T
    let mut stft: Option<StftBuf> = None;

    for n in &g.nodes {
        match n.op.as_str() {
            "W" => {
                // Параметры окна/хопа (MVP): n_fft=64 по умолчанию, hop=n_fft/2
                let n_fft = 64usize;
                let hop = n_fft / 2;
                let edge = n
                    .params
                    .get("edge")
                    .and_then(|v| v.as_str())
                    .unwrap_or("reflect");
                if edge != "reflect" && edge != "Toeplitz" {
                    bail!("waverunner: unsupported edge='{edge}' for W");
                }
                let win = hann(n_fft);
                let (mono, _) = extract_mono(&wf.tracks)?;
                let frames = stft_frames(&mono, n_fft, hop, edge);
                let mut spec = Vec::with_capacity(frames.len());
                for fr in frames {
                    let mut wfr = fr.clone();
                    apply_window_inplace(&mut wfr, &win);
                    spec.push(dft(&wfr));
                }
                stft = Some(StftBuf {
                    frames: spec,
                    n_fft,
                    hop,
                    win,
                    orig_len: mono.len(),
                });
            }
            "T" => {
                // Если нет STFT — NOP (сохраняем обратную совместимость)
                if let Some(buf) = stft.take() {
                    let y = istft_ola(&buf.frames, buf.n_fft, buf.hop, &buf.win, buf.orig_len)?;
                    // Обновим tracks.mono
                    let mut tracks = serde_json::Map::new();
                    tracks.insert(
                        "mono".into(),
                        Value::Array(y.iter().map(|x| Value::from(*x)).collect()),
                    );
                    // частота дискретизации не меняется
                    wf.tracks = Value::Object(tracks);
                }
            }
            "D" => {
                // ВАЖНО: если до этого был W и в stft есть кадры — сначала вернёмся во временную область,
                // чтобы D работал по правильному сигналу, и чтобы последующий T не затирал результат.
                if let Some(buf) = stft.take() {
                    let y = istft_ola(&buf.frames, buf.n_fft, buf.hop, &buf.win, buf.orig_len)?;
                    let mut tracks = serde_json::Map::new();
                    tracks.insert(
                        "mono".into(),
                        Value::Array(y.iter().map(|x| Value::from(*x)).collect()),
                    );
                    wf.tracks = Value::Object(tracks);
                }

                let lambda = n
                    .params
                    .get("lambda")
                    .and_then(|v| v.as_f64())
                    .or_else(|| n.params.get("λ").and_then(|v| v.as_f64()))
                    .ok_or_else(|| anyhow!("waverunner: D requires numeric 'lambda'"))?;
                let aa = n
                    .params
                    .get("aa")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("waverunner: D requires 'aa' string"))?;
                wf = downsample_with_aa(&wf, lambda, aa)?;
            }
            "WML" | "X" | "R" | "P" => bail!("waverunner: op '{}' not implemented", n.op),
            _ => { /* неизвестные операторы игнорим как NOP, чтобы не ломать окружение */ }
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

    let (mono, other_keys) = extract_mono(&wf.tracks)?;
    let lmb = lambda;
    let cutoff = 0.5 / lmb; // до Найквиста после D
    let taps = 31; // нечётное
    let h = make_lowpass_sinc_hann(cutoff, taps);

    let filtered = conv_reflect(&mono, &h);

    let step = lmb.round() as usize;
    let mut out = Vec::with_capacity((filtered.len() + step - 1) / step);
    let mut idx = 0usize;
    while idx < filtered.len() {
        out.push(filtered[idx]);
        idx += step;
    }

    let mut hdr = wf.header.clone();
    if let Some(rate) = hdr.rate {
        let new_rate = ((rate as f64) / lmb).round() as u32;
        hdr.rate = Some(new_rate.max(1));
    }

    let mut tracks = serde_json::Map::new();
    tracks.insert("mono".into(), Value::Array(out.iter().map(|x| Value::from(*x)).collect()));
    for (k, v) in other_keys {
        tracks.insert(k, v);
    }

    Ok(WaveForm {
        header: hdr,
        tracks: Value::Object(tracks),
        passports: wf.passports.clone(),
    })
}

/// Hann окно
fn hann(n: usize) -> Vec<f64> {
    if n <= 1 { return vec![1.0]; }
    (0..n).map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * (i as f64) / (n as f64 - 1.0)).cos()).collect()
}

fn apply_window_inplace(x: &mut [f64], w: &[f64]) {
    for (xi, wi) in x.iter_mut().zip(w.iter()) {
        *xi *= *wi;
    }
}

/// Формирование STFT-кадров с отражающими границами
fn stft_frames(x: &[f64], n_fft: usize, hop: usize, edge: &str) -> Vec<Vec<f64>> {
    let mut frames = Vec::new();
    let mut start = 0usize;
    while start < x.len() {
        let mut fr = Vec::with_capacity(n_fft);
        for k in 0..n_fft {
            let idx = start as isize + k as isize;
            fr.push(match edge {
                "Toeplitz" => sample_toeplitz(x, idx),
                _ => sample_reflect(x, idx),
            });
        }
        frames.push(fr);
        start += hop;
    }
    // гарантируем хотя бы один кадр
    if frames.is_empty() {
        frames.push(vec![0.0; n_fft]);
    }
    frames
}

/// iSTFT: overlap-add + нормировка суммой квадратов окна (COLA для Hann@50%)
fn istft_ola(spec: &[Vec<Complex>], n_fft: usize, hop: usize, win: &[f64], orig_len: usize) -> Result<Vec<f64>> {
    if spec.is_empty() { return Ok(vec![]); }
    let total = (spec.len() - 1) * hop + n_fft;
    let mut y = vec![0.0f64; total];
    let mut wsum = vec![0.0f64; total];

    for (i, x) in spec.iter().enumerate() {
        let mut fr = idft(x);
        // окно назад (в прямом мы умножали)
        apply_window_inplace(&mut fr, win);
        let start = i * hop;
        for k in 0..n_fft {
            y[start + k] += fr[k];
            wsum[start + k] += win[k] * win[k];
        }
    }
    for i in 0..total {
        if wsum[i] > 1e-12 {
            y[i] /= wsum[i];
        }
    }
    // обрежем до исходной длины
    Ok(y.into_iter().take(orig_len).collect())
}

/// Наивный DFT/IDFT (O(N^2)) — достаточно для acceptance
fn dft(x: &[f64]) -> Vec<Complex> {
    let n = x.len();
    let mut out = vec![Complex::new(0.0, 0.0); n];
    let two_pi = 2.0 * std::f64::consts::PI;
    for k in 0..n {
        let mut acc_re = 0.0;
        let mut acc_im = 0.0;
        for (n_i, &xn) in x.iter().enumerate() {
            let ang = -two_pi * (k as f64) * (n_i as f64) / (n as f64);
            acc_re += xn * ang.cos();
            acc_im += xn * ang.sin();
        }
        out[k] = Complex::new(acc_re, acc_im);
    }
    out
}
fn idft(x: &[Complex]) -> Vec<f64> {
    let n = x.len();
    let mut out = vec![0.0f64; n];
    let two_pi = 2.0 * std::f64::consts::PI;
    for n_i in 0..n {
        let mut acc_re = 0.0;
        for (k, &xk) in x.iter().enumerate() {
            let ang = two_pi * (k as f64) * (n_i as f64) / (n as f64);
            acc_re += xk.re * ang.cos() - xk.im * ang.sin();
        }
        out[n_i] = acc_re / (n as f64);
    }
    out
}

/// Извлекает массив mono и остальные ключи tracks
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
        let sinc = if x == 0.0 {
            2.0 * fc
        } else {
            (2.0 * std::f64::consts::PI * fc * x).sin() / (std::f64::consts::PI * x)
        };
        let w = 0.5 * (1.0 - (2.0 * std::f64::consts::PI * (n as f64) / (m as f64 - 1.0)).cos());
        h.push(sinc * w);
    }
    // нормализация
    let sum: f64 = h.iter().sum();
    let mut out = h;
    if sum.abs() > 1e-12 {
        for v in &mut out {
            *v /= sum;
        }
    }
    out
}

/// Свертка с отражающими границами
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

/// Отражающий доступ
fn sample_reflect(x: &[f64], idx: isize) -> f64 {
    let n = x.len() as isize;
    if n <= 0 { return 0.0; }
    if n == 1 { return x[0]; }
    let mut i = idx;
    if i < 0 { i = -i; }
    let period = 2 * (n - 1);
    i = i % period;
    if i >= n {
        i = period - i;
    }
    x[i as usize]
}

/// Toeplitz-экстраполяция (MVP: последнее значение)
fn sample_toeplitz(x: &[f64], idx: isize) -> f64 {
    if x.is_empty() { return 0.0; }
    if idx < 0 { x[0] } else if (idx as usize) >= x.len() { x[x.len() - 1] } else { x[idx as usize] }
}
