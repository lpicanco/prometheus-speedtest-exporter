use crate::speedtest::SpeedtestResult;
use prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct Gauge(IntGaugeVec);
impl Gauge {
    pub fn register(name: &str, help: &str) -> Self {
        Self(register_int_gauge_vec!(name, help, &["server_name", "server_id", "isp"]).unwrap())
    }
    pub fn set(&self, value: i64, speedtest_result: &SpeedtestResult) {
        let values = &[
            speedtest_result.server.name.as_str(),
            &format!("{}", speedtest_result.server.id),
            speedtest_result.isp.as_str(),
        ];
        self.0.with_label_values(values).set(value);
    }
}
