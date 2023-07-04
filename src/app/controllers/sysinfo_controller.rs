use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::app::informers::{
    cpu_informer::{get_cpu_info, CpuHarvest},
    disk_informer::get_disk_info,
    memory_informer::get_memory_info,
    network_informer::{get_network_info, NetworkHarvest},
    sys_informer::{get_boot_time, get_load_average, get_sysinfo, get_users},
    temperature_informer::{get_temperature_info, TemperatureHarvest},
};

#[derive(Serialize)]
pub struct CpuResponse {
    cpu_info: Option<Vec<CpuHarvest>>,
}

#[derive(Serialize)]
pub struct TemperatureResponse {
    temperature_info: Option<Vec<TemperatureHarvest>>,
}

pub async fn cpus() -> (StatusCode, Json<CpuResponse>) {
    let cpu_info = get_cpu_info();
    let response = CpuResponse { cpu_info };
    (StatusCode::OK, Json(response))
}

pub async fn disks() -> Response {
    let disk_info = get_disk_info();
    match disk_info {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}

pub async fn memory() -> Response {
    let memory_info = get_memory_info();
    match memory_info {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}

pub async fn temperatures() -> (StatusCode, Json<TemperatureResponse>) {
    let temperature_info = get_temperature_info();
    let response = TemperatureResponse { temperature_info };
    (StatusCode::OK, Json(response))
}

pub async fn networks() -> Response {
    let network_info = get_network_info();
    match network_info {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}

pub async fn sysinfo() -> Response {
    let sysinfo = get_sysinfo();
    match sysinfo {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}

pub async fn boot_time() -> (StatusCode, Json<u64>) {
    let boot_time = get_boot_time();
    (StatusCode::OK, Json(boot_time))
}

pub async fn load_average() -> Response {
    let load_average = get_load_average();
    match load_average {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}

pub async fn users() -> Response {
    let users = get_users();
    match users {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", err),
        )
            .into_response(),
    }
}
