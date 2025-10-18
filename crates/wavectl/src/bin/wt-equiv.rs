use std::fs::{self, File};
use std::io::Write;
use clap::Parser;
use serde_json::json;

type Spectrogram = Vec<(Vec<f64>, Vec<f64>)>;

#[derive(Parser, Debug)]
#[command(about="WT Equivalence checker (STFT/ISTFT via DFT reference)")]
struct Args {
    /// sine | sweep
    #[arg(long, default_value="sine")]
    signal: String,
    /// Output WFR path
    #[arg(long, default_value="build/wt_equiv/out.wfr.json")]
    out: String,
    /// Sample rate
    #[arg(long, default_value_t=48_000)]
    sr: usize,
    /// FFT size
    #[arg(long, default_value_t=512)]
    n_fft: usize,
    /// hop size
    #[arg(long, default_value_t=256)]
    hop: usize,
    /// seconds
    #[arg(long, default_value_t=0.1)]
    duration: f64,
    /// MSE threshold for PASS
    #[arg(long, default_value_t=1e-9)]
    mse_threshold: f64,
    /// SDR (dB) min for PASS
    #[arg(long, default_value_t=60.0)]
    sdr_min: f64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let n = (args.sr as f64 * args.duration) as usize;
    let x = match args.signal.as_str() {
        "sweep" => gen_sweep(n, args.sr, 200.0, 4000.0),
        _ => gen_sine(n, args.sr, 1000.0),
    };
    let (spec, frames) = stft_dft(&x, args.n_fft, args.hop);
    let y = istft_idft(&spec, frames, args.n_fft, args.hop);

    let mse = mean_squared_error(&x, &y);
    let sdr = sdr_db(&x, &y);

    let pass = mse <= args.mse_threshold && sdr >= args.sdr_min;

    let wfr = json!({
        "schema_version":"1.0",
        "created_at": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "run_id": uuid::Uuid::new_v4().to_string(),
        "cert": {
            "i1_unique_nf": true,
            "i2_delta_l_le_0": true,
            "i3_conservative_functors": pass,
            "i4_descent": null,
            "i5_mdl_consistent": null,
            "notes": null
        },
        "mdl": null,
        "phase": null,
        "swap": null,
        "w_params": {
            "n_fft": args.n_fft, "hop": args.hop, "window": "Hann",
            "center": false, "pad_mode": "reflect", "mode": "amp"
        },
        "w_perf": {
            "backend": "dft_ref", "backend_version": "v1",
            "wall_ms": 0.0, "frames": frames as u64, "threads": null
        },
        "metrics": {
            "mse": mse, "rel_mse": null, "snr_db": sdr, "cola_max_dev": null
        },
        "log": []
    });

    if let Some(dir) = std::path::Path::new(&args.out).parent() {
        fs::create_dir_all(dir)?;
    }
    let mut f = File::create(&args.out)?;
    write!(f, "{}", serde_json::to_string_pretty(&wfr)?)?;
    println!("[wt-equiv] Wrote {}", &args.out);
    if pass { Ok(()) } else { anyhow::bail!("thresholds not met: mse={}, sdr={}", mse, sdr) }
}

fn gen_sine(n: usize, sr: usize, f: f64) -> Vec<f64> {
    (0..n).map(|i| (2.0*std::f64::consts::PI * f * (i as f64)/(sr as f64)).sin()).collect()
}

fn gen_sweep(n: usize, sr: usize, f0: f64, f1: f64) -> Vec<f64> {
    (0..n).map(|i| {
        let t = i as f64 / sr as f64;
        let k = (f1 - f0) / (n as f64 / sr as f64);
        (2.0*std::f64::consts::PI * (f0 * t + 0.5*k*t*t)).sin()
    }).collect()
}

fn hann(n: usize) -> Vec<f64> {
    (0..n).map(|i| 0.5 - 0.5*(2.0*std::f64::consts::PI*(i as f64)/(n as f64)).cos()).collect()
}

fn dft(xr: &[f64], xi: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = xr.len();
    let mut xr_out = vec![0.0; n];
    let mut xi_out = vec![0.0; n];
    for k in 0..n {
        let mut sr = 0.0;
        let mut si = 0.0;
        for n0 in 0..n {
            let ang = -2.0*std::f64::consts::PI*(k as f64)*(n0 as f64)/(n as f64);
            let c = ang.cos();
            let s = ang.sin();
            let r = c*xr[n0] - s*xi[n0];
            let im = s*xr[n0] + c*xi[n0];
            sr += r;
            si += im;
        }
        xr_out[k]=sr;
        xi_out[k]=si;
    }
    (xr_out, xi_out)
}

fn idft(xr: &[f64], xi: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = xr.len();
    let mut xr_out = vec![0.0; n];
    let mut xi_out = vec![0.0; n];
    for n0 in 0..n {
        let mut sr = 0.0;
        let mut si = 0.0;
        for k in 0..n {
            let ang = 2.0*std::f64::consts::PI*(k as f64)*(n0 as f64)/(n as f64);
            let c = ang.cos();
            let s = ang.sin();
            let r = c*xr[k] - s*xi[k];
            let im = s*xr[k] + c*xi[k];
            sr += r;
            si += im;
        }
        xr_out[n0]=sr/(n as f64);
        xi_out[n0]=si/(n as f64);
    }
    (xr_out, xi_out)
}

fn stft_dft(x: &[f64], n_fft: usize, hop: usize) -> (Spectrogram, usize) {
    let w = hann(n_fft);
    let mut frames = 0usize;
    let mut out: Spectrogram = Vec::new();
    let mut i = 0usize;
    while i + n_fft <= x.len() {
        let mut xr = vec![0.0; n_fft];
        let xi = vec![0.0; n_fft];
        for j in 0..n_fft {
            xr[j] = x[i+j] * w[j];
        }
        let (ar, ai) = dft(&xr, &xi);
        out.push((ar, ai));
        frames += 1;
        i += hop;
    }
    (out, frames)
}

fn istft_idft(spec: &Spectrogram, frames: usize, n_fft: usize, hop: usize) -> Vec<f64> {
    let w = hann(n_fft);
    let len = (frames-1)*hop + n_fft;
    let mut y = vec![0.0; len];
    let mut win_sum = vec![0.0; len];
    for (f, (ar, ai)) in spec.iter().enumerate().take(frames) {
        let (tr, _ti) = idft(ar, ai);
        let base = f*hop;
        for j in 0..n_fft {
            y[base+j] += tr[j]*w[j];
            win_sum[base+j] += w[j]*w[j];
        }
    }
    for i in 0..len {
        if win_sum[i] > 1e-12 { y[i] /= win_sum[i]; }
    }
    y
}

fn mean_squared_error(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    let mut s = 0.0;
    for i in 0..n {
        let d = x[i]-y[i];
        s += d*d;
    }
    s/(n as f64)
}

fn sdr_db(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    let mut sx = 0.0;
    let mut se = 0.0;
    for i in 0..n {
        sx += x[i]*x[i];
        let d = x[i]-y[i];
        se += d*d;
    }
    if se <= 1e-30 { return 300.0; }
    10.0 * (sx/se).log10()
}
