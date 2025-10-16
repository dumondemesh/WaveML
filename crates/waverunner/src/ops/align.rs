//! Align operator: frame-wise isometric alignment by maximizing correlation.
#[derive(Clone, Copy, Debug)]
pub enum AlignMode { XCorrSoft, XCorrHard }

#[derive(Clone, Copy, Debug)]
pub struct AlignParams {
    pub mode: AlignMode,
    pub radius: usize
}

impl Default for AlignParams {
    fn default() -> Self { Self { mode: AlignMode::XCorrSoft, radius: 8 } }
}

/// In-place alignment of frames to the first frame (reference).
/// `frames` — набор действительных векторов одинаковой длины (например, |FFT|).
pub fn exec_align(frames: &mut [Vec<f64>], p: &AlignParams) {
    if frames.is_empty() { return; }
    let ref_frame = frames[0].clone();
    for f in frames.iter_mut().skip(1) {
        let s = best_shift_xcorr(&ref_frame, &*f, p.radius);
        apply_shift(f, s);
    }
}

// --- helpers ---

fn best_shift_xcorr(a: &[f64], b: &[f64], radius: usize) -> isize {
    let mut best_s = 0isize;
    let mut best_v = f64::NEG_INFINITY;
    for s in -(radius as isize)..=(radius as isize) {
        let v = xcorr_at(a, b, s);
        if v > best_v {
            best_v = v;
            best_s = s;
        }
    }
    best_s
}

fn xcorr_at(a: &[f64], b: &[f64], s: isize) -> f64 {
    // dot product of overlapping region, normalized by sqrt(Ea Eb)
    let (mut ia0, mut ib0) = if s >= 0 { (s, 0) } else { (0, -s) };
    let mut num = 0.0;
    let mut ea = 0.0;
    let mut eb = 0.0;
    while ia0 < a.len() as isize && ib0 < b.len() as isize {
        let va = a[ia0 as usize];
        let vb = b[ib0 as usize];
        num += va * vb;
        ea += va * va;
        eb += vb * vb;
        ia0 += 1;
        ib0 += 1;
    }
    if ea == 0.0 || eb == 0.0 { return 0.0; }
    num / (ea.sqrt() * eb.sqrt())
}

fn apply_shift(x: &mut Vec<f64>, s: isize) {
    if s == 0 { return; }
    if s > 0 { x.rotate_right(s as usize); } else { x.rotate_left((-s) as usize); }
}
