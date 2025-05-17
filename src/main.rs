use anyhow::Result;
use clap::Parser;
mod io_analyzer;
mod proc_scanner;
mod tui;
use crate::io_analyzer::IoAnalyzer;
use crate::proc_scanner::ProcScanner;

#[derive(Parser)]
#[command(author, version, about = "iotop - I/O monitor")]
struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,
    #[arg(long)]
    user: Option<String>,
    #[arg(long)]
    filter: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut tui = tui::Ui::new()?;
    let mut scanner = ProcScanner::new(cli.user, cli.filter);
    let mut analyzer = IoAnalyzer;
    tui.run(&mut scanner, &mut analyzer, cli.interval)?;
    Ok(())
}
