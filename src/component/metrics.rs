use serde::Serialize;
use std::borrow::Cow;
use sysinfo::{CpuExt, CpuRefreshKind, NetworkExt, NetworksExt, RefreshKind, System, SystemExt};
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct MetricsInfo<'a> {
    pub cpu_usage: Cow<'a, str>,
    pub memory_usage: Cow<'a, str>,
    pub network_in_bytes: u64,
    pub network_out_bytes: u64,
    pub network_in_packets: u64,
    pub network_out_packets: u64,
}

pub struct SystemMetrics {
    system: Mutex<System>,
}

impl SystemMetrics {
    pub fn new() -> SystemMetrics {
        let refresh_kind = RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory()
            .with_networks();

        let system = Mutex::new(sysinfo::System::new_with_specifics(refresh_kind));

        SystemMetrics { system }
    }

    pub async fn get(&self) -> MetricsInfo {
        let mut guard = self.system.lock().await;
        guard.refresh_cpu();
        guard.refresh_memory();
        guard.refresh_networks();

        let cpu_usage = guard.global_cpu_info().cpu_usage();

        let memory_usage = ((1.0
            - (guard.available_memory() as f64) / (guard.total_memory() as f64))
            * 100.0) as f32;

        let network_in_bytes = guard
            .networks()
            .iter()
            .fold(0, |current, (_, data)| current + data.received());

        let network_out_bytes = guard
            .networks()
            .iter()
            .fold(0, |current, (_, data)| current + data.transmitted());

        let network_in_packets = guard
            .networks()
            .iter()
            .fold(0, |current, (_, data)| current + data.packets_received());

        let network_out_packets = guard
            .networks()
            .iter()
            .fold(0, |current, (_, data)| current + data.packets_transmitted());

        MetricsInfo {
            cpu_usage: Cow::Owned(format!("{:.2}", cpu_usage)),
            memory_usage: Cow::Owned(format!("{:.2}", memory_usage)),
            network_in_bytes,
            network_out_bytes,
            network_in_packets,
            network_out_packets,
        }
    }
}
