use anyhow::Result;
use procfs::process::all_processes;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProcIo {
    pub pid: i32,
    pub cmd: String,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

#[derive(Default)]
pub struct ProcScanner {
    last_snapshot: HashMap<i32, (u64, u64)>, // pid -> (read, write)
}

impl ProcScanner {
    pub fn snapshot(&mut self) -> Result<Vec<ProcIo>> {
        let mut metrics = Vec::new();
        for prc_result in all_processes()? {
            let prc = match prc_result {
                Ok(p) => p,
                Err(_) => continue, // skip processes that fail to read
            };
            let io = match prc.io() {
                Ok(io) => io,
                Err(_) => continue,
            };
            let stat = match prc.stat() {
                Ok(stat) => stat,
                Err(_) => continue,
            };

            let read = io.read_bytes;
            let write = io.write_bytes;

            let (last_r, last_w) = self
                .last_snapshot
                .get(&prc.pid)
                .cloned()
                .unwrap_or((read, write));
            let delta_r = read.saturating_sub(last_r);
            let delta_w = write.saturating_sub(last_w);

            self.last_snapshot.insert(prc.pid, (read, write));

            metrics.push(ProcIo {
                pid: prc.pid,
                cmd: stat.comm,
                read_bytes: delta_r,
                write_bytes: delta_w,
            });
        }
        Ok(metrics)
    }
}
