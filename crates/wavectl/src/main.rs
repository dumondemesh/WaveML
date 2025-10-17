use anyhow::Result;
use clap::Parser;

mod cli;
mod cmd_cola;
mod cmd_forge;
mod cmd_report_from_graph;
mod cmd_simulate_swaps;
mod cmd_validate_wfr;

fn main() -> Result<()> {
    // Важно: парсить корневой тип Cli, а не enum Command
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::Cola {
            n_fft,
            hop,
            window,
            center,
            pad_mode,
            mode,
            out,
            ..
        } => {
            let window_s = window.to_string();
            cmd_cola::run(n_fft, hop, &window_s, center, &pad_mode, &mode, &out)
        }

        cli::Command::ValidateWfr { wfr, require_pass, .. } => {
            // актуальная сигнатура: (wfr_path, require_pass)
            cmd_validate_wfr::run(&wfr, require_pass)
        }

        // epsilon присутствует в структуре — явно игнорируем через `..`
        cli::Command::SimulateSwaps { input, out, .. } => {
            // актуальная сигнатура: (input, out)
            cmd_simulate_swaps::run(&input, &out)
        }

        // tol тоже есть — игнорируем через `..`
        cli::Command::ReportFromGraph { input, out, mode, .. } => {
            // актуальная сигнатура: (input, out, mode)
            cmd_report_from_graph::run(&input, &out, &mode)
        }

        cli::Command::Forge {
            input,
            print_id,
            print_nf,
            out,
        } => {
            // run принимает Option<&Path> для out
            cmd_forge::run(&input, print_id, print_nf, out.as_deref())
        }
    }
}
