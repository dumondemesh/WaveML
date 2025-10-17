// waveeval crate — minimal public surface used by wavectl
// Provides: ColaMode enum and cola_ok() checker.
//
// This version accepts either &str or ColaMode for `mode` via Into<ColaMode>
// and returns a real Error type so callers can use `?` with anyhow.

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColaMode {
    Amp,
    Power,
}

impl ColaMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColaMode::Amp => "amp",
            ColaMode::Power => "power",
        }
    }

    pub fn from<S: AsRef<str>>(s: S) -> Self {
        match s.as_ref().trim().to_lowercase().as_str() {
            "power" | "pow" => ColaMode::Power,
            _ => ColaMode::Amp,
        }
    }

    // Compatibility shim so existing code using `ColaMode::from_str(...)` compiles
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Self {
        Self::from(s)
    }
}

// Allow passing &str / String directly into generic cola_ok
impl From<&str> for ColaMode {
    fn from(s: &str) -> Self { ColaMode::from(s) }
}
impl From<String> for ColaMode {
    fn from(s: String) -> Self { ColaMode::from(s.as_str()) }
}

#[derive(Debug)]
pub struct WaveEvalError(pub String);
impl Display for WaveEvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
impl std::error::Error for WaveEvalError {}

fn hann_periodic(n_fft: usize) -> Vec<f64> {
    let n = n_fft as f64;
    (0..n_fft).map(|i| {
        let x = 2.0 * std::f64::consts::PI * (i as f64) / n;
        0.5 - 0.5 * x.cos()
    }).collect()
}

fn rect(n_fft: usize) -> Vec<f64> {
    vec![1.0; n_fft]
}

fn window_from_name(name: &str, n_fft: usize) -> Result<Vec<f64>, WaveEvalError> {
    match name.trim().to_lowercase().as_str() {
        "hann" | "hanning" => Ok(hann_periodic(n_fft)),
        "rect" | "boxcar" | "rectangular" => Ok(rect(n_fft)),
        other => Err(WaveEvalError(format!("unknown window '{other}'"))),
    }
}

/// Check COLA for a window/hop. Returns (ok, rel_dev, mean_sum).
/// `mode`: "amp" — overlap-add of window, "power" — overlap-add of power (w^2).
pub fn cola_ok<M: Into<ColaMode>>(
    n_fft: usize,
    hop: usize,
    window: &str,
    tol: f64,
    mode: M,
) -> Result<(bool, f64, f64), WaveEvalError> {
    if n_fft == 0 || hop == 0 {
        return Err(WaveEvalError("n_fft and hop must be > 0".to_string()));
    }
    let w = window_from_name(window, n_fft)?;
    let k = n_fft.div_ceil(hop);

    let mode_e: ColaMode = mode.into();
    let mut s = vec![0.0_f64; n_fft];

    match mode_e {
        ColaMode::Amp => {
            for (t, slot) in s.iter_mut().enumerate() {
                let mut acc = 0.0_f64;
                for j in 0..k {
                    let idx = (t + j * hop) % n_fft;
                    acc += w[idx];
                }
                *slot = acc;
            }
        }
        ColaMode::Power => {
            for (t, slot) in s.iter_mut().enumerate() {
                let mut acc = 0.0_f64;
                for j in 0..k {
                    let idx = (t + j * hop) % n_fft;
                    let v = w[idx];
                    acc += v * v;
                }
                *slot = acc;
            }
        }
    }

    let mean = s.iter().sum::<f64>() / (n_fft as f64);
    if mean == 0.0 {
        return Err(WaveEvalError("mean is zero; invalid configuration".to_string()));
    }
    let max_abs_dev = s.iter().map(|&x| (x - mean).abs()).fold(0.0_f64, f64::max);
    let rel = max_abs_dev / mean;
    Ok((rel <= tol, rel, mean))
}
