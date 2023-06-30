use axum::{http::StatusCode, response::IntoResponse};

use crate::app::informers::sys_informer::check_os_support;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404")
}

pub async fn handler_418() -> impl IntoResponse {
    (StatusCode::IM_A_TEAPOT, "I'm a teapot")
}

pub async fn handler_500() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn support_check() -> impl IntoResponse {
    let check_result = check_os_support();
    (StatusCode::OK, check_result.to_string())
}
