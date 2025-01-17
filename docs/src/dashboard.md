# Grafana Dashboard

A pre-configured Grafana dashboard is available to visualize your internet speed metrics.

## Dashboard Features

The dashboard includes:
- Download and upload speed trends
- Ping latency statistics
- Jitter measurements
- Historical performance graphs
- Server information

## Installation

The dashboard is available on Grafana.com: [Internet Speed Dashboard](https://grafana.com/grafana/dashboards/22634-internet-speed/)

You can import it using:
- Dashboard ID: `22634`
- Direct import from [Grafana.com marketplace](https://grafana.com/grafana/dashboards/22634-internet-speed/)

## Docker Compose Setup

If you're using our Docker Compose setup, the dashboard is automatically provisioned and ready to use.

1. Access Grafana at `http://localhost:3000`
2. Login with default credentials (admin/admin)
3. Navigate to Dashboards -> Internet Speed

## Screenshots

![Grafana Dashboard](https://raw.githubusercontent.com/lpicanco/prometheus-speedtest-exporter/refs/heads/main/docs/grafana_dashboard.png)

## Manual Setup

If you're setting up the dashboard manually:

1. Add Prometheus as a data source in Grafana
2. Import the dashboard using ID `22634`
3. Select your Prometheus data source
4. Save and enjoy your internet speed metrics! 