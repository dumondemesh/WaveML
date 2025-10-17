use anyhow::Result;

mod cli;
mod cmd_cola;
mod cmd_validate_wfr;
mod cmd_simulate_swaps;
mod cmd_report_from_graph;
mod cmd_forge;
mod cmd_nf_diff;
mod cmd_nf_batch;
mod cmd_forge_explain; // <= важно: explain у тебя, скорее всего, в отдельном модуле

use clap::Parser;

fn main() -> Result<()> {
    let cmd = <cli::Command as Parser>::parse();

    match cmd.cmd {
        cli::Cmd::Cola(a) => cmd_cola::run(
            a.n_fft,
            a.hop,
            &a.window,
            a.center,
            &a.pad_mode,
            &a.mode,
            &a.out,
        ),
        cli::Cmd::ValidateWfr(a) => cmd_validate_wfr::run(&a.wfr, a.require_pass),
        cli::Cmd::SimulateSwaps(a) => cmd_simulate_swaps::run(&a.input, &a.out),
        cli::Cmd::ReportFromGraph(a) => cmd_report_from_graph::run(&a.input, &a.out, &a.mode),
        cli::Cmd::Forge(a) => cmd_forge::run(&a.input, a.print_id, a.print_nf, a.out.as_deref(), a.check),
        cli::Cmd::ForgeExplain(a) => cmd_forge_explain::run(&a.input), // <-- вместо cmd_forge::explain
        cli::Cmd::NfDiff(a) => cmd_nf_diff::run(
            &a.left,
            &a.right,
            a.fail_on_diff,
            a.id_only,
            a.json,
            a.show_source_diff,
        ),
        cli::Cmd::NfBatch(a) => cmd_nf_batch::run(
            &a.inputs,
            a.list.as_ref(),
            a.json,
            a.csv,
            a.out.as_deref(),
        ),
    }
}
