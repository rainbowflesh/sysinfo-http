use anyhow::Result;
use std::{
    fs::{self, File},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn add_logger(level: String, log_file_path: &str) -> Result<()> {
    let logging_level = match level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        _ => panic!("Invalid level"),
    };
    match fs::create_dir_all(log_file_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error creating log file: {}", e);
        }
    }

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let filename = format!("{}{}.log", log_file_path, since_epoch.as_secs());
    let file = File::create(filename.clone())?;
    println!("Log file path: {:?}", filename);

    let log_layer = tracing_subscriber::fmt::layer();
    let log2file = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));
    let log2file_layer = log_layer.and_then(log2file);
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", logging_level)
        .with_target("tower_http::trace::on_request", logging_level)
        .with_target("tower_http::trace::make_span", logging_level)
        .with_default(Level::INFO);
    tracing_subscriber::registry()
        .with(log2file_layer)
        .with(filter)
        .init();
    Ok(())
}
