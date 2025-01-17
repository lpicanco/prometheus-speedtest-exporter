---
layout: default
---

# Overview

A Prometheus exporter that runs speedtest.net measurements and exports the results as metrics.

[![Build Status](https://github.com/lpicanco/prometheus-speedtest-exporter/workflows/build/badge.svg)](https://github.com/lpicanco/prometheus-speedtest-exporter/actions)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=lpicanco_prometheus-speedtest-exporter&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=lpicanco_prometheus-speedtest-exporter)
[![Version](https://img.shields.io/badge/version-0.2.3-blue.svg)](https://github.com/lpicanco/prometheus-speedtest-exporter/releases/tag/v0.2.3)

## Features

- Periodic speedtest measurements
- Prometheus metrics for:
  - Ping latency (average, low, high)
  - Download performance (bandwidth, bytes, elapsed time, latency)
  - Upload performance (bandwidth, bytes, elapsed time, latency)
- Configurable test intervals
- Multi-architecture support (amd64, arm64, armv7)
- Docker support
- Minimal resource footprint (<1MiB RAM usage)

![Memory Usage](https://raw.githubusercontent.com/lpicanco/prometheus-speedtest-exporter/refs/heads/main/docs/memory_usage.png)
*Container memory usage example*

## Quick Start

The fastest way to get started is using our docker-compose file which includes:
- Speedtest exporter (running tests every 30 minutes)
- Prometheus (pre-configured to scrape the metrics)
- Grafana (with auto-provisioned dashboard)

```bash
# Download the docker-compose.yml
curl -O https://raw.githubusercontent.com/lpicanco/prometheus-speedtest-exporter/main/docker-compose.yml

# Start the stack
docker-compose up -d
```

Then access Grafana at http://localhost:3000 (admin/admin) and you'll find:
- Pre-configured Prometheus data source
- Auto-provisioned Internet Speed dashboard
- Real-time metrics for download, upload, and ping latency

## Grafana Dashboard

A pre-configured Grafana dashboard is available to visualize your internet speed metrics:

[![Grafana Dashboard](https://raw.githubusercontent.com/lpicanco/prometheus-speedtest-exporter/refs/heads/main/docs/grafana_dashboard.png)](https://grafana.com/grafana/dashboards/22651-prometheus-speedtest-exporter/)

You can import this dashboard in two ways:
1. Using the Grafana.com dashboard ID: `22651`
2. Directly from [Grafana.com marketplace](https://grafana.com/grafana/dashboards/22651-prometheus-speedtest-exporter/)

## Documentation

For detailed documentation, please visit our [GitHub repository](https://github.com/lpicanco/prometheus-speedtest-exporter#readme). 