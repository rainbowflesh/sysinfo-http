use axum::{routing::get, Router};

use crate::app::controllers::sysinfo_controller;

pub fn append_sysinfo_route(base_router: Router) -> Router {
    base_router
        .route("/cpus", get(sysinfo_controller::cpus))
        .route("/disks", get(sysinfo_controller::disks))
        .route("/memory", get(sysinfo_controller::memory))
        .route("/networks", get(sysinfo_controller::networks))
        .route("/temperatures", get(sysinfo_controller::temperatures))
        .route("/load_average", get(sysinfo_controller::load_average))
        .route("/boot_time", get(sysinfo_controller::boot_time))
        .route("/sysinfo", get(sysinfo_controller::sysinfo))
        .route("/users", get(sysinfo_controller::users))
}
