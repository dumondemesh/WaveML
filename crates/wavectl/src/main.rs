    // WAVECTL_SENTINEL_v034_CLIPPY_PASS
    mod cli;
    mod cmd_validate_wfr;
    mod cmd_cola;
    mod cmd_simulate_swaps;
    mod cmd_report_from_graph;
    
    use anyhow::Result;
    use clap::Parser;
    
    fn main() -> Result<()> {
        // Parse CLI and init logging once
        let cli = cli::Cli::parse();
        wave_logging::init(&cli.log_level, &cli.log_format);
    
        match cli.cmd {
            cli::Command::ValidateWfr(a) => cmd_validate_wfr::run(&a.wfr, &a.schema, a.require_pass),
            cli::Command::Cola(a) => cmd_cola::run(a.n_fft, a.hop, &a.window.to_string(), a.center, &a.pad_mode, &a.mode, &a.out),
            cli::Command::SimulateSwaps(a) => cmd_simulate_swaps::run(&a.input, &a.out, a.epsilon),
            cli::Command::ReportFromGraph(a) => cmd_report_from_graph::run(&a.input, &a.out, &a.mode, a.tol),
            cli::Command::Acceptance(a) => {
                use anyhow::Context;
                use std::fs;
                #[derive(serde::Deserialize)] struct Plan { tests: Vec<Test> }
                #[derive(serde::Deserialize)] struct Test { name: String, cmd: String }
                let txt = fs::read_to_string(&a.plan).with_context(|| format!("read {:?}", a.plan))?;
                let plan: Plan = serde_yaml::from_str(&txt).context("parse plan yaml")?;
                std::fs::create_dir_all(&a.outdir).ok();
                // consume fields to satisfy clippy and produce human summary
                let mut lines = Vec::with_capacity(plan.tests.len());
                for t in &plan.tests {
                    lines.push(format!("- {} :: {}", t.name, t.cmd));
                }
                let summary = format!("# Acceptance — summary\n\n**Всего:** {}\n\n{}\n", plan.tests.len(), lines.join("\n"));
                fs::write(a.outdir.join("index.md"), summary).context("write summary")?;
                Ok(())
            }
        }
    }
