# Quick Start

The fastest way to get started is using our Docker Compose setup, which includes:
- Speedtest exporter (runs every 30 minutes)
- Prometheus (scrapes metrics every hour)
- Grafana (with pre-configured dashboard)

## Using Docker Compose

1. Download our Docker Compose file:
```bash
curl -o docker-compose.yml https://raw.githubusercontent.com/lpicanco/prometheus-speedtest-exporter/main/docker-compose.yml
```

2. Start the stack:
```bash
docker-compose up -d
```

That's it! Your monitoring stack is now running:
- Speedtest exporter: `http://localhost:9516/metrics`
- Prometheus: `http://localhost:9090`
- Grafana: `http://localhost:3000` (default credentials: admin/admin)

The Grafana dashboard will be automatically provisioned and ready to show your internet speed metrics. 