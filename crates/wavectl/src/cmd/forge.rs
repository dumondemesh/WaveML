use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use waveforge::{canonicalize_graph, nf_id_hex};

mod io;
use io::{read_inputs, InputItem};

#[derive(Parser, Debug)]
#[command(about="Canonicalize graphs and compute NF/NF-ID")]
pub struct Args {
    /// Input file(s). Use "-" for stdin; supports glob patterns.
    #[arg(long="input", required=true, num_args=1..)]
    pub input: Vec<String>,

    /// Print only NF-ID (hex) to stdout.
    #[arg(long="print-id", default_value_t=false)]
    pub print_id: bool,

    /// Print canonical NF JSON to stdout.
    #[arg(long="print-nf", default_value_t=false)]
    pub print_nf: bool,

    /// Validate that input is already canonical (exit 0 if yes).
    #[arg(long="check", default_value_t=false)]
    pub check: bool,
}

pub fn main_impl() -> Result<()> {
    let args = Args::parse();
    let items = read_inputs(&args.input)?;

    for item in items {
        let (src, text) = match item {
            InputItem::Stdin(s) => ("<stdin>".to_string(), s),
            InputItem::FilePath(p, s) => (p, s),
        };
        let v: Value = serde_json::from_str(&text)
            .with_context(|| format!("parse json failed: {src}"))?;

        if args.check {
            let canon = canonicalize_graph(&v)?;
            if &canon == &v {
                // already canonical: silent success
            } else {
                return Err(anyhow::anyhow!("input is not canonical (differs from canonical NF)"));
            }
        } else if args.print_nf {
            let canon = canonicalize_graph(&v)?;
            println!("{}", serde_json::to_string_pretty(&canon)?);
        } else {
            let id = nf_id_hex(&v)?;
            // Strict stdout: only hex on stdout
            println!("{}", id);
        }
    }
    Ok(())
}
