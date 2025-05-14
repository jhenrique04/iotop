use crate::proc_scanner::ProcIo;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Impact {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct AnalyzedProc {
    pub pid: i32,
    pub cmd: String,
    pub read_per_s: f64,
    pub write_per_s: f64,
    pub impact: Impact,
}

#[derive(Default)]
pub struct IoAnalyzer;

impl IoAnalyzer {
    pub fn evaluate(&mut self, data: Vec<ProcIo>) -> Vec<AnalyzedProc> {
        data.into_iter()
            .map(|p| {
                let read_mb = p.read_bytes as f64 / 1_000_000f64;
                let write_mb = p.write_bytes as f64 / 1_000_000f64;
                let impact = if read_mb + write_mb > 50.0 {
                    Impact::High
                } else if read_mb + write_mb > 5.0 {
                    Impact::Medium
                } else {
                    Impact::Low
                };
                AnalyzedProc {
                    pid: p.pid,
                    cmd: p.cmd,
                    read_per_s: read_mb,
                    write_per_s: write_mb,
                    impact,
                }
            })
            .collect()
    }
}
