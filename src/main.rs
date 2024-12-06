use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;
use crate::metrics::{register, register_int, FloatGauge, IntGauge};
use crate::speedtest::run_speedtest;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};
use prometheus::{Encoder, TextEncoder};
use tokio::task::spawn_blocking;
use tokio::time::interval;

mod config;
mod speedtest;
mod metrics;

type SharedState = Arc<Mutex<AppState>>;

struct AppState {
    ping_latency_gauge: FloatGauge,
    ping_low_gauge: FloatGauge,
    ping_high_gauge: FloatGauge,

    download_bytes_gauge: IntGauge,
    download_bandwidth_bytes_gauge: IntGauge,
    download_elapsed_seconds_gauge: FloatGauge,
    download_latency_iqm_seconds_gauge: FloatGauge,
    download_latency_low_seconds_gauge: FloatGauge,
    download_latency_high_seconds_gauge: FloatGauge,

    upload_bytes_gauge: IntGauge,
    upload_bandwidth_bytes_gauge: IntGauge,
    upload_elapsed_seconds_gauge: FloatGauge,
    upload_latency_iqm_seconds_gauge: FloatGauge,
    upload_latency_low_seconds_gauge: FloatGauge,
    upload_latency_high_seconds_gauge: FloatGauge,
}

impl AppState {
    fn new() -> Self {
        Self {
            ping_latency_gauge: register("speedtest_ping_latency_seconds", "Speedtest ping latency in seconds"),
            ping_low_gauge: register("speedtest_ping_low_seconds", "Speedtest ping low in seconds"),
            ping_high_gauge: register("speedtest_ping_high_seconds", "Speedtest ping high in seconds"),

            download_bytes_gauge: register_int("speedtest_download_bytes", "Number of bytes downloaded during speedtest"),
            download_bandwidth_bytes_gauge: register_int("speedtest_download_bandwidth_bytes", "Speedtest download bandwidth in bytes per second"),
            download_elapsed_seconds_gauge: register("speedtest_download_elapsed_seconds", "Speedtest download elapsed time in seconds"),
            download_latency_iqm_seconds_gauge: register("speedtest_download_latency_iqm_seconds", "Speedtest download latency iqm in seconds"),
            download_latency_low_seconds_gauge: register("speedtest_download_latency_low_seconds", "Speedtest download latency low in seconds"),
            download_latency_high_seconds_gauge: register("speedtest_download_latency_high_seconds", "Speedtest download latency high in seconds"),

            upload_bytes_gauge: register_int("speedtest_upload_bytes", "Number of bytes uploaded during speedtest"),
            upload_bandwidth_bytes_gauge: register_int("speedtest_upload_bandwidth_bytes", "Speedtest upload bandwidth in bytes per second"),
            upload_elapsed_seconds_gauge: register("speedtest_upload_elapsed_seconds", "Speedtest upload elapsed time in seconds"),
            upload_latency_iqm_seconds_gauge: register("speedtest_upload_latency_iqm_seconds", "Speedtest upload latency iqm in seconds"),
            upload_latency_low_seconds_gauge: register("speedtest_upload_latency_low_seconds", "Speedtest upload latency low in seconds"),
            upload_latency_high_seconds_gauge: register("speedtest_upload_latency_high_seconds", "Speedtest upload latency high in seconds"),
        }
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let config = Config::parse();
    debug!("Loaded configuration: {:?}", config);

    let addr = format!("{}:{}", config.http_host, config.http_port);
    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await.unwrap();

    let shared_state = SharedState::new(Mutex::new(AppState::new()));

    tokio::spawn(
        speedtest_task(config, shared_state.clone())
    );

    let app = Router::new()
        .route("/metrics", get(handle_metrics))
        .with_state(shared_state);

    info!("🦀Server running at https://{}", &addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
        })
        .await.unwrap();
}

async fn speedtest_task(config: Config, shared_state: SharedState) {
    let mut interval = interval(Duration::from_secs(config.test_interval_minutes * 60));
    loop {
        interval.tick().await;

        match spawn_blocking(run_speedtest).await.expect("Failed to spawn task") {
            Ok(result) => {
                let app_state = shared_state.lock().unwrap();
                app_state.ping_latency_gauge.set(result.ping.latency_seconds(), &result);
                app_state.ping_low_gauge.set(result.ping.low_seconds(), &result);
                app_state.ping_high_gauge.set(result.ping.high_seconds(), &result);
                
                app_state.download_bytes_gauge.set(result.download.bytes, &result);
                app_state.download_bandwidth_bytes_gauge.set(result.download.bandwidth, &result);
                app_state.download_elapsed_seconds_gauge.set(result.download.elapsed_seconds(), &result);
                app_state.download_latency_iqm_seconds_gauge.set(result.download.latency_iqm_seconds(), &result);
                app_state.download_latency_low_seconds_gauge.set(result.download.latency_low_seconds(), &result);
                app_state.download_latency_high_seconds_gauge.set(result.download.latency_high_seconds(), &result);

                app_state.upload_bytes_gauge.set(result.upload.bytes, &result);
                app_state.upload_bandwidth_bytes_gauge.set(result.upload.bandwidth, &result);
                app_state.upload_elapsed_seconds_gauge.set(result.upload.elapsed_seconds(), &result);
                app_state.upload_latency_iqm_seconds_gauge.set(result.upload.latency_iqm_seconds(), &result);
                app_state.upload_latency_low_seconds_gauge.set(result.upload.latency_low_seconds(), &result);
                app_state.upload_latency_high_seconds_gauge.set(result.upload.latency_high_seconds(), &result);
            }
            Err(e) => {
                error!("Failed to run speedtest: {}", e);
            }
        }
    }
}

async fn handle_metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    (axum::http::StatusCode::OK, buffer)
}

#[cfg(test)]
mod tests {
    use speedtest::SpeedtestResult;
    use super::*;
    use std::{fs, sync::Mutex};

    fn set_metrics(app_state: &AppState, result: &SpeedtestResult) {
        // Set ping metrics
        app_state.ping_latency_gauge.set(result.ping.latency_seconds(), result);
        app_state.ping_low_gauge.set(result.ping.low_seconds(), result);
        app_state.ping_high_gauge.set(result.ping.high_seconds(), result);
        
        // Set download metrics
        app_state.download_bytes_gauge.set(result.download.bytes, result);
        app_state.download_bandwidth_bytes_gauge.set(result.download.bandwidth, result);
        app_state.download_elapsed_seconds_gauge.set(result.download.elapsed_seconds(), result);
        app_state.download_latency_iqm_seconds_gauge.set(result.download.latency_iqm_seconds(), result);
        app_state.download_latency_low_seconds_gauge.set(result.download.latency_low_seconds(), result);
        app_state.download_latency_high_seconds_gauge.set(result.download.latency_high_seconds(), result);

        // Set upload metrics
        app_state.upload_bytes_gauge.set(result.upload.bytes, result);
        app_state.upload_bandwidth_bytes_gauge.set(result.upload.bandwidth, result);
        app_state.upload_elapsed_seconds_gauge.set(result.upload.elapsed_seconds(), result);
        app_state.upload_latency_iqm_seconds_gauge.set(result.upload.latency_iqm_seconds(), result);
        app_state.upload_latency_low_seconds_gauge.set(result.upload.latency_low_seconds(), result);
        app_state.upload_latency_high_seconds_gauge.set(result.upload.latency_high_seconds(), result);
    }

    fn get_metrics_output() -> String {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }

    #[tokio::test]
    async fn test_speedtest_metrics() {
        // Initialize app state
        let app_state = AppState::new();
        let shared_state = SharedState::new(Mutex::new(app_state));

        // Load and parse test data
        let json_str = fs::read_to_string("tests/test_data.json")
            .expect("Failed to read test data file");
        let result: SpeedtestResult = serde_json::from_str(&json_str)
            .expect("Failed to parse test data");

        // Set metrics using test data
        {
            let app_state = shared_state.lock().unwrap();
            set_metrics(&app_state, &result);
        }

        // Get metrics output
        let metrics_output = get_metrics_output();

        // Verify expected metrics
        let expected_metrics = [
            ("speedtest_ping_latency_seconds{isp=\"Test ISP\",server_id=\"52533\",server_name=\"Virtual Machines\"} 0.01228", "ping latency"),
            ("speedtest_download_bandwidth_bytes{isp=\"Test ISP\",server_id=\"52533\",server_name=\"Virtual Machines\"} 39924051", "download bandwidth"),
            ("speedtest_upload_bandwidth_bytes{isp=\"Test ISP\",server_id=\"52533\",server_name=\"Virtual Machines\"} 13008272", "upload bandwidth"),
        ];

        for (metric, description) in expected_metrics {
            assert!(
                metrics_output.contains(metric),
                "Missing or incorrect {} metric",
                description
            );
        }
    }
}