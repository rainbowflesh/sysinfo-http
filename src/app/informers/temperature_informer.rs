use serde::Serialize;
use sysinfo::{ComponentExt, System, SystemExt};

#[derive(Default, Clone, Debug, Serialize)]
pub struct TemperatureHarvest {
    pub name: String,
    pub temperature: f32,
}

pub fn get_temperature_info() -> Option<Vec<TemperatureHarvest>> {
    let mut temperature_vec: Vec<TemperatureHarvest> = Vec::new();
    let mut sys = System::new_all();
    sys.refresh_components();
    let sensor_data = sys.components();
    for component in sensor_data {
        let name = component.label().to_string();
        temperature_vec.push(TemperatureHarvest {
            name,
            temperature: { component.temperature() },
        });
    }
    Some(temperature_vec)
}

pub fn get_temperature_as_json() -> String {
    let mut sys = System::new_all();
    sys.refresh_components();
    serde_json::to_string(&sys).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_temperature_info() {
        let result = get_temperature_info();
        println!("{:?}", result);
    }

    #[test]
    fn test_get_temperature_as_json() {
        let temper = get_temperature_as_json();
        println!("{:?}", temper)
    }
}
