//! FFT backend (rustfft) for STFT/ISTFT used by W/T_inv.
use rustfft::{FftPlanner, num_complex::Complex};
use crate::ops::w::{WParams, WindowKind};

pub type Spectrogram = Vec<Vec<Complex<f64>>>;

pub fn stft_rustfft(x: &[f64], p: &WParams) -> Spectrogram {
    let n = p.n_fft;
    let hop = p.hop;
    let win = make_window(n, p.window);
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(n);

    let frames_est = x.len().saturating_sub(n).saturating_add(hop) / hop + 1;
    let mut frames = Vec::with_capacity(frames_est);

    let mut i = 0usize;
    while i + n <= x.len() {
        let mut buf: Vec<Complex<f64>> =
            x[i..i+n].iter().zip(win.iter()).map(|(xi, w)| Complex::new(xi * w, 0.0)).collect();
        fft.process(&mut buf);
        frames.push(buf);
        i += hop;
    }
    frames
}

pub fn istft_rustfft(spec: &[Vec<Complex<f64>>], p: &WParams, out_len: usize) -> Vec<f64> {
    let n = p.n_fft;
    let hop = p.hop;
    let win = make_window(n, p.window);

    let mut planner = FftPlanner::<f64>::new();
    let ifft = planner.plan_fft_inverse(n);

    let mut y = vec![0.0f64; out_len];
    let mut norm = vec![0.0f64; out_len];

    for (k, frame) in spec.iter().enumerate() {
        let mut buf = frame.clone();
        ifft.process(&mut buf);
        let t0 = k * hop;
        for j in 0..n {
            if t0 + j >= out_len { break; }
            // COLA overlap-add
            let val = (buf[j].re / n as f64) * win[j];
            y[t0 + j] += val;
            norm[t0 + j] += win[j] * win[j];
        }
    }
    for i in 0..out_len {
        if norm[i] > 0.0 {
            y[i] /= norm[i].sqrt(); // unbiased COLA
        }
    }
    y
}

pub fn make_window(n: usize, kind: WindowKind) -> Vec<f64> {
    match kind {
        WindowKind::Hann => (0..n)
            .map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos())
            .collect(),
        WindowKind::Hamming => (0..n)
            .map(|i| 0.54 - 0.46 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos())
            .collect(),
        WindowKind::Blackman => (0..n)
            .map(|i| {
                let a0 = 0.42;
                let a1 = 0.5;
                let a2 = 0.08;
                let t = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                a0 - a1 * t.cos() + a2 * (2.0 * t).cos()
            })
            .collect(),
    }
}
