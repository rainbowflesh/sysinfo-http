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
    temperature_informer::{get_temperature_info, TemperatureHarvest},
};

#[derive(Serialize)]
pub struct TemperatureResponse {
    temperature_info: Option<Vec<TemperatureHarvest>>,
}

#[derive(Serialize)]
pub struct CpuResponse {
    cpu_info: Option<Vec<CpuHarvest>>,
}

#[derive(Serialize)]
pub struct NetworkResponse {
    network_info: Option<Vec<NetworkHarvest>>,
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
