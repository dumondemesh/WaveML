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
            bank: "hann-default".to_string(),
            n_fft: 1024,
            hop: 512,
            window: WindowKind::Hann,
            center: true,
            pad_mode: PadMode::Reflect,
        }
    }
}

pub fn exec_w(x: &[f64], p: &WParams) -> Spectrogram {
    // R7: zero-pad forbidden; only reflect is allowed at edges
    match p.pad_mode {
        PadMode::Reflect => {}
    }
    stft_rustfft(x, p)
}

// (доп. хелпер — если нужно локально)
pub fn make_window(n: usize, w: WindowKind) -> Vec<f64> {
    match w {
        WindowKind::Hann => (0..n).map(|i| {
            let a = std::f64::consts::TAU * (i as f64) / (n as f64);
            0.5 - 0.5 * a.cos()
        }).collect(),
        WindowKind::Hamming => (0..n).map(|i| {
            let a = std::f64::consts::TAU * (i as f64) / (n as f64);
            0.54 - 0.46 * a.cos()
        }).collect(),
        WindowKind::Blackman => (0..n).map(|i| {
            let a = std::f64::consts::TAU * (i as f64) / (n as f64);
            0.42 - 0.5 * a.cos() + 0.08 * (2.0 * a).cos()
        }).collect(),
    }
}
