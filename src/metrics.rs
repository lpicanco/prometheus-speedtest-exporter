use prometheus::{IntGaugeVec, register_int_gauge_vec};
use crate::speedtest::SpeedtestResult;

pub struct Gauge(IntGaugeVec);
impl Gauge {
    pub fn set(&self, value: i64, speedtest_result: &SpeedtestResult) {
        self.0.with_label_values(&[speedtest_result.server.host.as_str()]).set(value);
    }
}

pub fn register_gauge(name: &str, help: &str) -> Gauge {
    Gauge(register_int_gauge_vec!(name, help, &["host"]).unwrap())
}
