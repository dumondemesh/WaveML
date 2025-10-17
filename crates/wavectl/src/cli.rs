use clap::{Args, Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wavectl")]
#[command(about = "WaveML CLI utilities", long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Check COLA analytically and write a minimal WFR report
    Cola(ColaArgs),

    /// Validate a WFR file (v1 schema). Optional: require overall pass
    #[command(name = "validate-wfr")]
    ValidateWfr(ValidateWfrArgs),

    /// Simulate swaps and produce a WFR skeleton
    #[command(name = "simulate-swaps")]
    SimulateSwaps(SimulateSwapsArgs),

    /// Produce a WFR "report" from a graph (toy)
    #[command(name = "report-from-graph")]
    ReportFromGraph(ReportFromGraphArgs),

    /// Canonicalize graph to NF and/or print NF-ID
    Forge(ForgeArgs),

    /// Explain how graph was canonicalized (per-node diffs) and show NF-ID
    #[command(name = "forge-explain")]
    ForgeExplain(ForgeExplainArgs),

    /// Compare two inputs by their canonical NF (prints NF-IDs and a diff)
    #[command(name = "nf-diff")]
    NfDiff(NfDiffArgs),

    /// Batch-compute NF-ID for many inputs (JSON/CSV/human)
    #[command(name = "nf-batch")]
    NfBatch(NfBatchArgs),
}

/* ===== args for existing commands ===== */

#[derive(Args, Debug)]
pub struct ColaArgs {
    #[arg(long)]
    pub n_fft: u32,
    #[arg(long)]
    pub hop: u32,
    #[arg(long, default_value = "Hann")]
    pub window: String,
    #[arg(long, default_value_t = true)]
    pub center: bool,
    #[arg(long, default_value = "reflect")]
    pub pad_mode: String,
    #[arg(long, default_value = "amp")]
    pub mode: String,
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub out: PathBuf,
}

#[derive(Args, Debug)]
pub struct ValidateWfrArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub wfr: PathBuf,
    #[arg(long, default_value_t = false)]
    pub require_pass: bool,
}

#[derive(Args, Debug)]
pub struct SimulateSwapsArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub input: PathBuf,
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub out: PathBuf,
}

#[derive(Args, Debug)]
pub struct ReportFromGraphArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub input: PathBuf,
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub out: PathBuf,
    #[arg(long, default_value = "amp")]
    pub mode: String,
}

#[derive(Args, Debug)]
pub struct ForgeArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub input: PathBuf,
    #[arg(long, default_value_t = false)]
    pub print_id: bool,
    #[arg(long, default_value_t = false)]
    pub print_nf: bool,
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub out: Option<PathBuf>,
    /// Return non-zero if input isn't canonical
    #[arg(long, default_value_t = false)]
    pub check: bool,
}

#[derive(Args, Debug)]
pub struct ForgeExplainArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub input: PathBuf,
}

#[derive(Args, Debug)]
pub struct NfDiffArgs {
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub left: PathBuf,
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub right: PathBuf,
    /// Exit with code 1 when NF differ
    #[arg(long, default_value_t = false)]
    pub fail_on_diff: bool,
    /// Print only NF-IDs (for scripts)
    #[arg(long, default_value_t = false)]
    pub id_only: bool,
    /// Print json summary (with optional source diff)
    #[arg(long, default_value_t = false)]
    pub json: bool,
    /// Also show source (pre-canonical) JSON diff
    #[arg(long, default_value_t = false)]
    pub show_source_diff: bool,
}

/* ===== NEW: nf-batch ===== */

#[derive(Args, Debug)]
pub struct NfBatchArgs {
    /// Inputs (can be given multiple times)
    #[arg(long = "input", value_hint = ValueHint::FilePath)]
    pub inputs: Vec<PathBuf>,

    /// Text file with list of inputs (blank lines/comments starting with # are ignored)
    #[arg(long = "list", value_hint = ValueHint::FilePath)]
    pub list: Option<PathBuf>,

    /// Output JSON to stdout
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Output CSV (to stdout unless --out given)
    #[arg(long, default_value_t = false)]
    pub csv: bool,

    /// Where to write CSV (optional)
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub out: Option<PathBuf>,
}
