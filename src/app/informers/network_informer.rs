use anyhow::Result;
use serde::Serialize;
use sysinfo::{NetworkExt, System, SystemExt};

#[derive(Default, Clone, Debug, Serialize)]
pub struct NetworkHarvest {
    pub interface_name: String,
    pub data_received: u64,
    pub data_transmitted: u64,
}

pub fn get_network_info() -> Result<Option<Vec<NetworkHarvest>>> {
    let mut result: Vec<NetworkHarvest> = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_networks();
    for (interface_name, data) in sys.networks() {
        result.push(NetworkHarvest {
            interface_name: interface_name.clone(),
            data_received: data.received(),
            data_transmitted: data.transmitted(),
        });
    }
    Ok(Some(result))
}

pub fn get_network_summary() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_network_info() {
        let res = get_network_info();
        println!("{:?}", res);
    }
}
