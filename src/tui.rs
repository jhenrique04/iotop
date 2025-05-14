use crate::io_analyzer::IoAnalyzer;
use crate::proc_scanner::ProcScanner;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

use crate::io_analyzer::{AnalyzedProc, Impact};

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn run(
        &mut self,
        scanner: &mut ProcScanner,
        analyzer: &mut IoAnalyzer,
        refresh_ms: u64,
    ) -> Result<()> {
        loop {
            let start = Instant::now();

            let raw_metrics = scanner.snapshot()?;
            let mut metrics = analyzer.evaluate(raw_metrics);

            self.draw(&mut metrics)?;

            if event::poll(Duration::from_millis(refresh_ms))? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }

            let elapsed = start.elapsed();
            if elapsed < Duration::from_millis(refresh_ms) {
                std::thread::sleep(Duration::from_millis(refresh_ms) - elapsed);
            }
        }
        Ok(())
    }

    pub fn draw(&mut self, procs: &mut [AnalyzedProc]) -> Result<()> {
        procs.sort_by(|a, b| {
            (b.read_per_s + b.write_per_s)
                .partial_cmp(&(a.read_per_s + a.write_per_s))
                .unwrap()
        });

        self.terminal.draw(|f| {
            let size = f.size();
            let rows: Vec<Row> = procs
                .iter()
                .map(|p| {
                    Row::new(vec![
                        Cell::from(p.pid.to_string()),
                        Cell::from(p.cmd.clone()),
                        Cell::from(format!("{:.1} MB/s", p.read_per_s)),
                        Cell::from(format!("{:.1} MB/s", p.write_per_s)),
                        Cell::from(match p.impact {
                            Impact::High => "HIGH",
                            Impact::Medium => "MED",
                            Impact::Low => "LOW",
                        }),
                    ])
                })
                .collect();

            let widths = [
                Constraint::Length(6),
                Constraint::Percentage(40),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(8),
            ];

            let table = Table::new(rows, widths)
                .header(
                    Row::new(["PID", "CMD", "READ/s", "WRITE/s", "IMPACT"])
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                )
                .block(Block::default().title("IOMap").borders(Borders::ALL));
            f.render_widget(table, size);
        })?;
        Ok(())
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).ok();
        self.terminal.show_cursor().ok();
    }
}
