use clap::{Parser, Subcommand, Args, ValueEnum};
use std::path::PathBuf;
use std::fmt::{Display, Formatter};

#[derive(Parser, Debug)]
#[command(name="wavectl", version, about="WaveML control CLI (minimal, WFR v1.x)")]
pub struct Cli {
    /// Log level: trace|debug|info|warn|error
    #[arg(long, env="WAVE_LOG", default_value="info")]
    pub log_level: String,
    /// Log format: compact|full|json
    #[arg(long, env="WAVE_LOG_FORMAT", default_value="compact")]
    pub log_format: String,
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Validate WFR file against v1.x schema (lightweight checks)
    ValidateWfr(ValidateWfr),
    /// Run COLA pipeline placeholder and emit WFR v1.x
    Cola(Cola),
    /// Simulate swaps for I2/I3 acceptance and emit WFR
    SimulateSwaps(SimulateSwaps),
    /// Build report from execution graph (placeholder)
    ReportFromGraph(ReportFromGraph),
    /// Run acceptance plan (YAML) and build bundle (placeholder)
    Acceptance(Acceptance),
}

#[derive(Args, Debug)]
pub struct ValidateWfr {
    #[arg(long)]
    pub wfr: PathBuf,
    #[arg(long, default_value="docs/schemas/wfr.v1.schema.json")]
    pub schema: PathBuf,
    #[arg(long, default_value_t=false)]
    pub require_pass: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Window { Hann, Hamming, Blackman }

impl Display for Window {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Window::Hann => write!(f, "Hann"),
            Window::Hamming => write!(f, "Hamming"),
            Window::Blackman => write!(f, "Blackman"),
        }
    }
}

#[derive(Args, Debug)]
pub struct Cola {
    #[arg(long)] pub n_fft: u32,
    #[arg(long)] pub hop: u32,
    #[arg(long, value_enum, ignore_case = true)] pub window: Window,
    #[arg(long, default_value_t=true)] pub center: bool,
    #[arg(long, default_value="reflect")] pub pad_mode: String,
    #[arg(long, default_value="amp")] pub mode: String,
    #[arg(long, default_value="build/reports/auto_amp.wfr.json")] pub out: PathBuf,
}

#[derive(Args, Debug)]
pub struct SimulateSwaps {
    #[arg(long)] pub input: PathBuf,
    #[arg(long, default_value="build/i23_pass.wfr.json")] pub out: PathBuf,
    #[arg(long, default_value_t=1e-6)] pub epsilon: f64,
}

#[derive(Args, Debug)]
pub struct ReportFromGraph {
    #[arg(long)] pub input: PathBuf,
    #[arg(long, default_value="build/reports/report.wfr.json")] pub out: PathBuf,
    #[arg(long, default_value="amp")] pub mode: String,
    #[arg(long, default_value_t=1e-12)] pub tol: f64,
}

#[derive(Args, Debug)]
pub struct Acceptance {
    #[arg(long, default_value="acceptance/tests.yaml")] pub plan: PathBuf,
    #[arg(long, default_value="build/acceptance")] pub outdir: PathBuf,
    #[arg(long, default_value_t=false)] pub strict: bool,
}
