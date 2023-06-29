use anyhow::Result;
use serde::Serialize;
use sysinfo::{System, SystemExt};
#[derive(Default, Clone, Debug, Serialize)]
pub struct MemoryHarvest {
    pub available_memory: u64,
    pub free_memory: u64,
    pub free_swap: u64,
    pub total_memory: u64,
    pub total_swap: u64,
    pub used_memory: u64,
    pub used_swap: u64,
}

pub fn get_memory_info() -> Result<Option<Vec<MemoryHarvest>>, anyhow::Error> {
    let mut result: Vec<MemoryHarvest> = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_memory();
    result.push(MemoryHarvest {
        available_memory: sys.available_memory(),
        free_memory: sys.free_memory(),
        free_swap: sys.free_swap(),
        total_memory: sys.total_memory(),
        total_swap: sys.total_swap(),
        used_memory: sys.used_memory(),
        used_swap: sys.used_swap(),
    });
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_memory_info() {
        todo!("implement");
    }
}
