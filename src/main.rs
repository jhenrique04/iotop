mod io_analyzer;
mod proc_scanner;
mod tui;

use anyhow::Result;
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about = "IOMap - Intelligent I/O monitor for Linux")]
struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut tui = tui::Ui::new()?;
    let scanner = proc_scanner::ProcScanner::default;
    let analyzer = io_analyzer::IoAnalyzer::default;

    tui.run(&mut scanner(), &mut analyzer(), cli.interval)?;

    Ok(())
}
