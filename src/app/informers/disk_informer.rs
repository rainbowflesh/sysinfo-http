use anyhow::Result;
use serde::Serialize;
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Default, Clone, Debug, Serialize)]
pub struct DiskHarvest {
    device_name: String,
    file_system: String,
    total_space: u64,
    available_space: u64,
}

pub fn get_disk_info() -> Result<Option<Vec<DiskHarvest>>> {
    let mut result: Vec<DiskHarvest> = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_disks();
    let disks = sys.disks();
    for disk in disks {
        result.push(DiskHarvest {
            device_name: disk.name().to_str().unwrap_or_default().to_string(),
            file_system: String::from_utf8(disk.file_system().to_vec())?,
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        })
    }
    Ok(Some(result))
}

pub fn get_disk_usage() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_disk_info() {
        let result = get_disk_info();
        println!("{:?}", result);
    }
}
