use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use clap::Parser;
use dotenv::dotenv;
use log::{debug, error, info};
use prometheus::{Encoder, TextEncoder};
use tokio::task::spawn_blocking;
use tokio::time::interval;
use crate::config::Config;
use crate::metrics::{Gauge, register_gauge};
use crate::speedtest::SpeedtestResult;

mod config;
mod speedtest;
mod metrics;

type SharedState = Arc<Mutex<AppState>>;

struct AppState {
    speedtest_download_bytes_gauge: Gauge,
    speedtest_download_bandwidth_bytes_gauge: Gauge,
    speedtest_download_duration_seconds_gauge: Gauge,
    speedtest_upload_bytes_gauge: Gauge,
    speedtest_upload_bandwidth_bytes_gauge: Gauge,
    speedtest_upload_duration_seconds_gauge: Gauge,
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

    let shared_state = SharedState::new(Mutex::new(
        AppState {
            speedtest_download_bytes_gauge: register_gauge("speedtest_download_bytes", "Number of bytes downloaded during speedtest"),
            speedtest_download_bandwidth_bytes_gauge: register_gauge("speedtest_download_bandwidth_bytes", "Speedtest download bandwidth in bytes/s"),
            speedtest_download_duration_seconds_gauge: register_gauge("speedtest_download_duration_seconds", "Speedtest download duration in seconds"),

            speedtest_upload_bytes_gauge: register_gauge("speedtest_upload_bytes", "Number of bytes uploaded during speedtest"),
            speedtest_upload_bandwidth_bytes_gauge: register_gauge("speedtest_upload_bandwidth_bytes", "Speedtest upload bandwidth in bytes/s"),
            speedtest_upload_duration_seconds_gauge: register_gauge("speedtest_upload_duration_seconds", "Speedtest upload duration in seconds"),
        }
    ));

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

async fn speedtest_task(config: Config, shared_state: SharedState) { // -> impl Future<Output=_> + Sized {
    let mut interval = interval(Duration::from_secs(config.test_interval_minutes * 60));
    loop {
        interval.tick().await;
        if let Ok(Ok(result)) = spawn_blocking(run_speedtest).await {
            let app_state = shared_state.lock().unwrap();
            app_state.speedtest_download_bytes_gauge.set(result.download.bytes, &result);
            app_state.speedtest_download_bandwidth_bytes_gauge.set(result.download.bandwidth, &result);
            app_state.speedtest_download_duration_seconds_gauge.set(result.download.elapsed, &result);

            app_state.speedtest_upload_bytes_gauge.set(result.upload.bytes, &result);
            app_state.speedtest_upload_bandwidth_bytes_gauge.set(result.upload.bandwidth, &result);
            app_state.speedtest_upload_duration_seconds_gauge.set(result.upload.elapsed, &result);
        } else if let Err(e) = spawn_blocking(run_speedtest).await {
            error!("Failed to run speedtest: {}", e);
        }
    }
}

fn run_speedtest() -> Result<SpeedtestResult, std::io::Error> {
    debug!("Running speedtest");
    let output = Command::new("speedtest")
        .arg("--format=json")
        .arg("--accept-license")
        .arg("--accept-gdpr")
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        SpeedtestResult::from_json(&output_str).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, output.status.to_string()))
    }
}

async fn handle_metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    (axum::http::StatusCode::OK, buffer)
}
