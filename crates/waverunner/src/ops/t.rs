//! Time-domain inverse for W: ISTFT using rustfft backend.
use crate::ops::fft::istft_rustfft;
use crate::ops::w::WParams;
use crate::ops::fft::Spectrogram;

pub fn exec_t_inv(spec: &Spectrogram, p_w: &WParams, out_len: usize) -> Vec<f64> {
    istft_rustfft(spec, p_w, out_len)
}
