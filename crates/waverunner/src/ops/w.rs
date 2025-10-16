//! Wave operator W: STFT with rustfft backend, R7-safe edges (reflect-only).
use crate::ops::fft::{stft_rustfft, Spectrogram};

#[derive(Clone, Copy, Debug)]
pub enum WindowKind { Hann, Hamming, Blackman }

#[derive(Clone, Copy, Debug)]
pub enum PadMode { Reflect }

#[derive(Clone, Debug)]
pub struct WParams {
    pub bank: String,
    pub n_fft: usize,
    pub hop: usize,
    pub window: WindowKind,
    pub center: bool,
    pub pad_mode: PadMode,
}

impl Default for WParams {
    fn default() -> Self {
        Self {
            bank: "hann-default".into(),
            n_fft: 1024,
            hop: 256,
            window: WindowKind::Hann,
            center: true,
            pad_mode: PadMode::Reflect,
        }
    }
}

pub fn exec_w(input: &[f64], p: &WParams) -> Spectrogram {
    // R7 guard: only reflect padding; no zero-padding allowed.
    let x = if p.center {
        let pad = p.n_fft / 2;
        reflect_pad(input, pad)
    } else {
        input.to_vec()
    };
    stft_rustfft(&x, p)
}

// --- helpers ---

fn reflect_pad(x: &[f64], m: usize) -> Vec<f64> {
    if m == 0 || x.is_empty() { return x.to_vec(); }
    let mut y = Vec::with_capacity(x.len() + 2 * m);
    // left reflect
    for i in (0..m).rev() {
        y.push(x[i.min(x.len() - 1)]);
    }
    // center
    y.extend_from_slice(x);
    // right reflect
    let n = x.len();
    for i in 0..m {
        let idx = n - 1 - i.min(n - 1);
        y.push(x[idx]);
    }
    y
}
