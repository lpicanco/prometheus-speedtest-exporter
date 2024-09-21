use serde::{Deserialize, Serialize};

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
    latency: LatencyResult
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    id: u64,
    name: String,
    location: String,
    country: String,
    pub host: String,
    port: u16,
    ip: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpeedtestResult {
    ping: PingResult,
    pub download: TestResult,
    pub upload: TestResult,
    pub server: Server,
}

impl SpeedtestResult {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}