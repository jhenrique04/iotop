# ```IOMap``` — I/O Monitor for Linux

```IOMap``` is a lightweight terminal-based I/O monitor for Linux that identifies processes causing high disk read/write usage and estimates their impact on system responsiveness.

## Features

### Small memory and CPU usage!

```IOMap``` is lightweight and efficient, minimizing memory and CPU usage by using optimized system calls and reading data from the /proc filesystem. 

It uses Rust and ```ratatui``` for a clean, responsive terminal UI that refreshes at a configurable interval.

### Real-time per-process disk I/O monitoring
It tracks read and write bytes per process in real-time, calculating deltas and showing throughput in MB/s.

### Impact estimation
Based on throughput thresholds, ```IOMap``` estimates the impact level of each process on system responsiveness (Low, Medium, High).

## Packaging

None yet :(

Contributions are welcome!

## Building

Requirements:
* [Rust toolchain](https://rustup.rs/)
* Linux system with ```/proc``` filesystem

```
git clone https://github.com/jhenrique04/iomap
cd iomap
cargo run --release 1000 # Put the interval that you want in ms here (1000 is the default)
```

## Prebuilt binaries

Binaries are generated at every commit through [GitHub Actions](https://github.com/vrmiguel/bustd/actions)

## TODO

- [x]  Basic per-process I/O monitoring and display
- [x]  Impact estimation heuristics
- [ ]  More advanced heuristics (including iowait and latency)
- [ ]  Filtering by user or process name
- [ ]  Non-TUI mode for scripts and servers
