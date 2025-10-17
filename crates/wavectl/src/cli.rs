use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name="wavectl", version, about="WaveML CLI")]
pub struct Cli {
    /// Log level: trace|debug|info|warn|error
    #[arg(long, env="WAVE_LOG", default_value="info")]
    pub log_level: String,
    /// Log format: compact|full|json
    #[arg(long, env="WAVE_LOG_FORMAT", default_value="compact")]
    pub log_format: String,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Check COLA condition and emit WFR
    Cola {
        #[arg(long)]
        n_fft: u32,
        #[arg(long)]
        hop: u32,
        #[arg(long, default_value="hann")]
        window: String,
        #[arg(long, default_value_t=true)]
        center: bool,
        #[arg(long, default_value="reflect")]
        pad_mode: String,
        #[arg(long, default_value="amp")]
        mode: String,
        #[arg(long)]
        out: PathBuf,
    },
    /// Validate WFR file against minimal pass policy
    ValidateWfr {
        #[arg(long)]
        wfr: PathBuf,
        #[arg(long, default_value_t=false)]
        require_pass: bool,
    },
    /// Simulate epsilon-swaps and emit WFR
    SimulateSwaps {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value_t=0.0)]
        epsilon: f64,
    },
    /// Build a report from graph (demo stub, emits WFR)
    ReportFromGraph {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value="amp")]
        mode: String,
        #[arg(long, default_value_t=1e-12)]
        tol: f64,
    },
    /// Compute STRICT-NF and/or NF-ID from a manifest JSON
    Forge {
        #[arg(long)]
        input: PathBuf,
        /// Print NF-ID to stdout
        #[arg(long, default_value_t=false)]
        print_id: bool,
        /// Print normalized JSON (STRICT-NF) to stdout
        #[arg(long, default_value_t=false)]
        print_nf: bool,
        /// Write normalized JSON to this file
        #[arg(long)]
        out: Option<PathBuf>,
    }
}
