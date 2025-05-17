use anyhow::Result;
use procfs::process::all_processes;
use std::collections::HashMap;
use std::fs;

pub struct ProcIo {
    pub pid: i32,
    pub cmd: String,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

pub struct ProcScanner {
    last_snapshot: HashMap<i32, (u64, u64)>,
    user_filter: Option<String>,
    cmd_filter: Option<String>,
    user_uids: HashMap<String, String>,
}

impl ProcScanner {
    pub fn new(user_filter: Option<String>, cmd_filter: Option<String>) -> Self {
        // build map username -> uid string
        let mut user_uids = HashMap::new();
        if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
            for line in passwd.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    user_uids.insert(parts[0].to_string(), parts[2].to_string());
                }
            }
        }
        Self {
            last_snapshot: HashMap::new(),
            user_filter,
            cmd_filter,
            user_uids,
        }
    }

    pub fn snapshot(&mut self) -> Result<Vec<ProcIo>> {
        let mut metrics = Vec::new();
        for entry in all_processes()? {
            let prc = match entry {
                Ok(p) => p,
                Err(_) => continue,
            };

            // user filter
            if let Some(ref user) = self.user_filter {
                if let Some(uid_expected) = self.user_uids.get(user) {
                    let status_path = format!("/proc/{}/status", prc.pid);
                    let status_text = fs::read_to_string(&status_path).unwrap_or_default();
                    let mut real_uid = "";
                    for line in status_text.lines() {
                        if line.starts_with("Uid:") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 2 {
                                real_uid = parts[1];
                            }
                            break;
                        }
                    }
                    if real_uid != uid_expected {
                        continue;
                    }
                } else {
                    // unknown user name
                    continue;
                }
            }

            let io = match prc.io() {
                Ok(i) => i,
                Err(_) => continue,
            };
            let stat = match prc.stat() {
                Ok(s) => s,
                Err(_) => continue,
            };

            // command substring filter
            if let Some(ref f) = self.cmd_filter {
                if !stat.comm.contains(f) {
                    continue;
                }
            }

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
