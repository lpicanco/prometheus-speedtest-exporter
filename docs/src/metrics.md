# Metrics

The exporter provides the following Prometheus metrics:

## Speed Metrics

| Metric | Description | Type |
|--------|-------------|------|
| `speedtest_download_bits_per_second` | Download speed in bits per second | Gauge |
| `speedtest_upload_bits_per_second` | Upload speed in bits per second | Gauge |

## Latency Metrics

| Metric | Description | Type |
|--------|-------------|------|
| `speedtest_ping_latency_milliseconds` | Ping latency in milliseconds | Gauge |
| `speedtest_jitter_latency_milliseconds` | Jitter (latency variation) in milliseconds | Gauge |

## Server Information

| Metric | Description | Type |
|--------|-------------|------|
| `speedtest_server_info` | Information about the speedtest server used | Info |
| `speedtest_client_info` | Information about the client (ISP, location) | Info |

## Operational Metrics

| Metric | Description | Type |
|--------|-------------|------|
| `speedtest_test_duration_seconds` | Duration of the speedtest in seconds | Gauge |
| `speedtest_test_error` | Error status of the last test (1 = error, 0 = success) | Gauge |

## Example Metrics Output

```text
# HELP speedtest_download_bits_per_second Download speed in bits per second
# TYPE speedtest_download_bits_per_second gauge
speedtest_download_bits_per_second 100000000

# HELP speedtest_upload_bits_per_second Upload speed in bits per second
# TYPE speedtest_upload_bits_per_second gauge
speedtest_upload_bits_per_second 50000000

# HELP speedtest_ping_latency_milliseconds Ping latency in milliseconds
# TYPE speedtest_ping_latency_milliseconds gauge
speedtest_ping_latency_milliseconds 15.5
``` 