use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::Config;
use crate::metrics::Gauge;
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
    download_bytes_gauge: Gauge,
    download_bandwidth_bytes_gauge: Gauge,
    download_elapsed_seconds_gauge: Gauge,
    upload_bytes_gauge: Gauge,
    upload_bandwidth_bytes_gauge: Gauge,
    upload_elapsed_seconds_gauge: Gauge,
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
            download_bytes_gauge: Gauge::register("speedtest_download_bytes", "Number of bytes downloaded during speedtest"),
            download_bandwidth_bytes_gauge: Gauge::register("speedtest_download_bandwidth_bytes", "Speedtest download bandwidth in bytes per second"),
            download_elapsed_seconds_gauge: Gauge::register("speedtest_download_elapsed_seconds", "Speedtest download elapsed time in seconds"),

            upload_bytes_gauge: Gauge::register("speedtest_upload_bytes", "Number of bytes uploaded during speedtest"),
            upload_bandwidth_bytes_gauge: Gauge::register("speedtest_upload_bandwidth_bytes", "Speedtest upload bandwidth in bytes per second"),
            upload_elapsed_seconds_gauge: Gauge::register("speedtest_upload_elapsed_seconds", "Speedtest upload elapsed time in seconds"),
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

async fn speedtest_task(config: Config, shared_state: SharedState) {
    let mut interval = interval(Duration::from_secs(config.test_interval_minutes * 60));
    loop {
        interval.tick().await;

        match spawn_blocking(run_speedtest).await.expect("Failed to spawn task") {
            Ok(result) => {
                let app_state = shared_state.lock().unwrap();
                app_state.download_bytes_gauge.set(result.download.bytes, &result);
                app_state.download_bandwidth_bytes_gauge.set(result.download.bandwidth, &result);
                app_state.download_elapsed_seconds_gauge.set(result.download.elapsed, &result);

                app_state.upload_bytes_gauge.set(result.upload.bytes, &result);
                app_state.upload_bandwidth_bytes_gauge.set(result.upload.bandwidth, &result);
                app_state.upload_elapsed_seconds_gauge.set(result.upload.elapsed, &result);
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
