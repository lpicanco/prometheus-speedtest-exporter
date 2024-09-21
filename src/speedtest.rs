use log::debug;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct LatencyResult {
    iqm: f64,
    low: f64,
    high: f64,
    jitter: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PingResult {
    jitter: f64,
    latency: f64,
    low: f64,
    high: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestResult {
    pub bandwidth: i64,
    pub bytes: i64,
    pub elapsed: i64,
    latency: LatencyResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: u64,
    pub name: String,
    location: String,
    country: String,
    host: String,
    port: u16,
    ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpeedtestResult {
    ping: PingResult,
    pub download: TestResult,
    pub upload: TestResult,
    pub server: Server,
    pub isp: String,
}

impl SpeedtestResult {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

pub fn run_speedtest() -> Result<SpeedtestResult, std::io::Error> {
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