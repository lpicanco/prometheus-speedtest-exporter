use crate::speedtest::SpeedtestResult;
use prometheus::core::{Atomic, AtomicF64, AtomicI64, GenericGaugeVec};
use prometheus::{register_gauge_vec, register_int_gauge_vec};

pub type IntGauge = Gauge<AtomicI64>;
pub type FloatGauge = Gauge<AtomicF64>;

pub struct Gauge<T: Atomic>(GenericGaugeVec<T>);

pub fn register_int(name: &str, help: &str) -> IntGauge {
    Gauge(register_int_gauge_vec!(name, help, &["server_name", "server_id", "isp"]).unwrap())
}

pub fn register(name: &str, help: &str) -> FloatGauge {
    Gauge(register_gauge_vec!(name, help, &["server_name", "server_id", "isp"]).unwrap())
}

impl<T: Atomic> Gauge<T> {
    pub fn set(&self, value: T::T, speedtest_result: &SpeedtestResult) {
        let values = &[
            speedtest_result.server.name.as_str(),
            &format!("{}", speedtest_result.server.id),
            speedtest_result.isp.as_str(),
        ];
        self.0.with_label_values(values).set(value);
    }
}
