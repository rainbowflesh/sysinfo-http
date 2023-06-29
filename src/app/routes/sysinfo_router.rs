use axum::{routing::get, Router};

use crate::app::controllers::sysinfo_controller;

pub fn append_sysinfo_route(base_router: Router) -> Router {
    base_router
        .route("/cpus", get(sysinfo_controller::cpus))
        .route("/disks", get(sysinfo_controller::disks))
        .route("/memory", get(sysinfo_controller::memory))
        .route("/temperatures", get(sysinfo_controller::temperatures))
}
