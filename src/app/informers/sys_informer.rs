use sysinfo::SystemExt;

pub fn check_os_support() -> bool {
    let is_supported = [<sysinfo::System as SystemExt>::IS_SUPPORTED];
    is_supported[0]
}

pub fn get_all_sysinfo() {
    todo!("implement");
}

pub fn get_boot_time() {
    todo!("implement");
}

pub fn get_up_time() {
    todo!("implement");
}

pub fn get_users() {
    todo!("implement");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_os_is_supported() {
        let is_supported = check_os_support();
        println!("Does my OS support lib sysinfo? {}", is_supported);
    }
}
