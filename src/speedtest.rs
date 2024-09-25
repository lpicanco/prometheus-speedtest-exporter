use log::debug;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
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
    pub latency: f64,
    pub low: f64,
    pub high: f64,
}

impl PingResult {
    pub fn latency_seconds(&self) -> f64 {
        self.latency / 1000.0
    }

    pub fn low_seconds(&self) -> f64 {
        self.low / 1000.0
    }

    pub fn high_seconds(&self) -> f64 {
        self.high / 1000.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestResult {
    pub bandwidth: i64,
    pub bytes: i64,
    pub elapsed: i64,
    latency: LatencyResult,
}

impl TestResult {
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed as f64 / 1000.0
    }
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
    pub ping: PingResult,
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
        SpeedtestResult::from_json(&output_str).map_err(|e| Error::new(ErrorKind::Other, e))
    } else {
        Err(Error::new(ErrorKind::Other, output.status.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_elapsed_seconds() {
        let test_result = TestResult {
            bandwidth: 1000,
            bytes: 2000,
            elapsed: 3200,
            latency: LatencyResult {
                iqm: 5.0,
                low: 1.0,
                high: 10.0,
                jitter: 2.0,
            },
        };

        assert_eq!(test_result.elapsed_seconds(), 3.2);
    }

    #[test]
    fn test_speedtest_result_from_json() {
        let json_str = fs::read_to_string("tests/test_data.json")
            .expect("Failed to read test data file");

        let result = SpeedtestResult::from_json(&json_str);

        assert!(result.is_ok());

        let speedtest_result = result.unwrap();
        assert_eq!(speedtest_result.ping.latency_seconds(), 0.01228);
        assert_eq!(speedtest_result.ping.low_seconds(), 0.012192);
        assert_eq!(speedtest_result.ping.high_seconds(), 0.012837);

        assert_eq!(speedtest_result.download.bandwidth, 39924051);
        assert_eq!(speedtest_result.download.bytes, 306775755);
        assert_eq!(speedtest_result.download.elapsed_seconds(), 7.6);
        assert_eq!(speedtest_result.upload.bandwidth, 13008272);
        assert_eq!(speedtest_result.server.name, "Virtual Machines");
        assert_eq!(speedtest_result.isp, "Test ISP");
    }
}