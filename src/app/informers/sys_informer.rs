use anyhow::Result;
use serde::Serialize;
use sysinfo::{System, SystemExt, UserExt};

#[derive(Default, Clone, Debug, Serialize)]
pub struct SysHarvest {
    pub kernel_version: String,
    pub os_version: String,
    pub long_os_version: String,
    pub distribution_id: String,
    pub host_name: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct UserHarvest {
    pub name: String,
    pub group: Vec<String>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct LoadAverageHarvest {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

pub fn check_os_support() -> bool {
    let is_supported = [<sysinfo::System as SystemExt>::IS_SUPPORTED];
    is_supported[0]
}

pub fn get_all() {
    todo!();
}

pub fn get_sysinfo() -> Result<Option<Vec<SysHarvest>>> {
    let mut result: Vec<SysHarvest> = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_networks();
    result.push(SysHarvest {
        kernel_version: sys.kernel_version().unwrap_or_default(),
        os_version: sys.os_version().unwrap_or_default(),
        long_os_version: sys.long_os_version().unwrap_or_default(),
        distribution_id: sys.distribution_id(),
        host_name: sys.host_name().unwrap_or_default(),
    });
    Ok(Some(result))
}

pub fn get_boot_time() -> u64 {
    let sys = System::new_all();
    let boot_time = sys.boot_time();
    boot_time
}

pub fn get_load_average() -> Result<Option<Vec<LoadAverageHarvest>>> {
    let sys = System::new_all();
    let load_avg = sys.load_average();
    let mut result: Vec<LoadAverageHarvest> = Vec::new();
    result.push(LoadAverageHarvest {
        one: load_avg.one,
        five: load_avg.five,
        fifteen: load_avg.fifteen,
    });
    Ok(Some(result))
}

pub fn get_users() -> Result<Option<Vec<UserHarvest>>> {
    let mut result: Vec<UserHarvest> = Vec::new();
    let mut sys = System::new_all();
    for user in sys.users() {
        result.push(UserHarvest {
            name: user.name().to_string(),
            group: user.groups().to_vec(),
        });
    }
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_os_is_supported() {
        let is_supported = check_os_support();
        println!("Does my OS support lib sysinfo? {}", is_supported);
    }

    #[test]
    fn test_get_users() {
        let res = get_users();
        println!("res: {:?}", res);
    }

    #[test]
    fn test_load_average() {
        let res = get_load_average();
        println!("res: {:?}", res);
    }

    #[test]
    fn test_boot_time() {
        let res = get_boot_time();
        println!("res: {:?}", res);
    }

    #[test]
    fn test_sysinfo() {
        let res = get_sysinfo();
        println!("res: {:?}", res);
    }
}
