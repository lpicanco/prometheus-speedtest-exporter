# Configuration

The exporter can be configured using environment variables:

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SPEEDTEST_INTERVAL` | Interval between speedtests in minutes | `30` |
| `SPEEDTEST_SERVER_ID` | Specific speedtest.net server ID to use | Not set (auto) |
| `LISTEN_ADDRESS` | Address to listen on | `0.0.0.0:9516` |

## Docker Configuration

When using Docker, you can pass environment variables using the `-e` flag:
```bash
docker run -p 9516:9516 \
  -e SPEEDTEST_INTERVAL=15 \
  -e SPEEDTEST_SERVER_ID=1234 \
  ghcr.io/lpicanco/prometheus-speedtest-exporter:latest
```

## Docker Compose Configuration

In your `docker-compose.yml`:
```yaml
services:
  speedtest-exporter:
    image: ghcr.io/lpicanco/prometheus-speedtest-exporter:latest
    environment:
      - SPEEDTEST_INTERVAL=15
      - SPEEDTEST_SERVER_ID=1234
    ports:
      - "9516:9516"
``` 