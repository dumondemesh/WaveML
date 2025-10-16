//! FFT backend (rustfft) for STFT/ISTFT used by W/T_inv.
use rustfft::{FftPlanner, num_complex::Complex};
use crate::ops::w::{WParams, WindowKind};

pub type Spectrogram = Vec<Vec<Complex<f64>>>;

/// STFT with reflect padding and windowing.
/// - center=true: reflect-pad by n_fft/2 at both sides
/// - center=false: no pad (assumes pre-aligned stream)
pub fn stft_rustfft(x: &[f64], p: &WParams) -> Spectrogram {
    let n = p.n_fft;
    let hop = p.hop;
    let win = make_window(n, p.window);
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(n);

    let frames_est = x.len().saturating_sub(n).saturating_add(hop) / hop + 1;
    let mut frames = Vec::with_capacity(frames_est);

    let xx: Vec<f64> = if p.center { reflect_pad(x, n / 2) } else { x.to_vec() };
    let mut buf: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];

    let mut i = 0usize;
    while i + n <= xx.len() {
        for k in 0..n {
            buf[k].re = xx[i + k] * win[k];
            buf[k].im = 0.0;
        }
        fft.process(&mut buf);
        frames.push(buf.clone());
        i += hop;
    }
    frames
}

/// ISTFT with overlap-add and sum-of-squares window normalization (COLA-safe).
pub fn istft_rustfft(spec: &[Vec<Complex<f64>>], p: &WParams, out_len: usize) -> Vec<f64> {
    let n = p.n_fft;
    let hop = p.hop;
    let win = make_window(n, p.window);

    let mut planner = FftPlanner::<f64>::new();
    let ifft = planner.plan_fft_inverse(n);

    let mut y = vec![0.0f64; out_len];
    let mut norm = vec![0.0f64; out_len];

    let mut frame_idx = 0usize;
    for frame in spec {
        let mut buf = frame.clone();
        ifft.process(&mut buf);
        let t0 = frame_idx * hop;
        for k in 0..n {
            let t = t0 + k;
            if t < y.len() {
                let v = (buf[k].re / (n as f64)) * win[k];
                y[t] += v;
                norm[t] += win[k] * win[k];
            }
        }
        frame_idx += 1;
    }

    // COLA normalization
    for t in 0..y.len() {
        let s = norm[t];
        if s > 1e-14 {
            y[t] /= s;
        }
    }

    // if center=true, stft added reflect-pad; trim it symmetrically
    if p.center {
        let m = n / 2;
        if y.len() >= 2 * m {
            y[m..(y.len() - m)].to_vec()
        } else {
            y
        }
    } else {
        y
    }
}

/// Make analysis/synthesis window.
fn make_window(n: usize, w: WindowKind) -> Vec<f64> {
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
            // Blackman (classic): a0=0.42 a1=0.5 a2=0.08
            let a = std::f64::consts::TAU * (i as f64) / (n as f64);
            0.42 - 0.5 * a.cos() + 0.08 * (2.0 * a).cos()
        }).collect(),
    }
}

/// Reflect padding by m samples on both sides.
fn reflect_pad(x: &[f64], m: usize) -> Vec<f64> {
    if m == 0 || x.is_empty() { return x.to_vec(); }
    let mut y = Vec::with_capacity(x.len() + 2 * m);
    // left reflect
    for i in (0..m).rev() {
        y.push(x[i.min(x.len() - 1)]);
    }
