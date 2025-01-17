# Installation

## Prerequisites
- [Ookla Speedtest CLI](https://www.speedtest.net/apps/cli) version 1.2.0 or higher

## Docker (Recommended)
```bash
docker run -p 9516:9516 ghcr.io/lpicanco/prometheus-speedtest-exporter:latest
```

## Binary Installation

1. Download the latest release for your platform from the [releases page](https://github.com/lpicanco/prometheus-speedtest-exporter/releases)

2. Extract and run:
```bash
chmod +x prometheus-speedtest-exporter
./prometheus-speedtest-exporter
```

## Building from Source

1. Install Rust and Cargo
2. Clone and build:
```bash
git clone https://github.com/lpicanco/prometheus-speedtest-exporter.git
cd prometheus-speedtest-exporter
cargo build --release
```

The binary will be available at `target/release/prometheus-speedtest-exporter` 