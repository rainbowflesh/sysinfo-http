use serde::Serialize;
use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};

#[derive(Default, Clone, Debug, Serialize)]
pub struct CpuHarvest {
    pub cpu_num: String,
    pub percent: f32,
    pub frequency: u64,
}

pub fn get_cpu_info() -> Option<Vec<CpuHarvest>> {
    let mut cpu_vec: Vec<CpuHarvest> = Vec::new();

    // cpu usage = new usage - old usage, so refresh twice
    let mut sys = System::new();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL); // and also double interval
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());

    for processor in sys.cpus() {
        cpu_vec.push(CpuHarvest {
            cpu_num: processor.name().to_string(),
            percent: processor.cpu_usage(),
            frequency: processor.frequency(),
        })
    }
    Some(cpu_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_usage() {
        let result = get_cpu_info();
        println!("{:?}", result);
    }
}
