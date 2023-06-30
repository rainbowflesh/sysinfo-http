use crate::app::routes::base_router::create_base_router;
use crate::app::routes::sysinfo_router::append_sysinfo_route;
use crate::app::utils::cli_helper::{handle_start_args, CliArgs};
use crate::app::utils::logger::add_logger;
use axum::Server;
#[cfg(unix)]
use tokio::sync::oneshot::{Receiver, Sender};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start_server() {
    let args = handle_start_args();
    #[cfg(unix)]
    {
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        init_server(args, shutdown_rx).await;
        handle_signal(shutdown_tx).await;
    }
    #[cfg(not(unix))]
    {
        init_server(args).await;
    }
}

#[cfg(unix)]
async fn handle_signal(shutdown_tx: Sender<()>) {
    use tokio::signal::unix::{signal, SignalKind};
    let signal_handler = tokio::spawn(async {
        tokio::pin! {
          let interrupt = signal(SignalKind::interrupt()).expect("could not open SIGINT channel");
          let quit = signal(SignalKind::quit()).expect("could not open SIGQUIT channel");
          let term = signal(SignalKind::terminate()).expect("could not open SIGTERM channel");
        };
        loop {
            tokio::select! {
              _ = (&mut interrupt).recv() => {
                  // info!("SIGINT received");
                  break;
              }
              _ = (&mut quit).recv() => {
                  // info!("SIGQUIT received");
                  break;
              }
              _ = (&mut term).recv() => {
                  // info!("SIGTERM received");
                  break;
              }
            }
        }
        shutdown_tx
            .send(())
            .expect("could not send shutdown signal");
    });
    signal_handler
        .await
        .expect("error with shutdown handler task");
}

#[cfg(unix)]
async fn init_server(args: CliArgs, shutdown_rx: Receiver<()>) {
    let bind_url = &args.bind;
    let log_level = args.log_level;
    let log_path = &args.log_path;
    add_logger(log_level.clone(), log_path).expect("Unable to create logger");
    let cors = CorsLayer::new().allow_origin(Any);
    let base_router = create_base_router();
    let app = append_sysinfo_route(base_router)
        .layer(cors)
        .layer(TraceLayer::new_for_http());
    info!("Server running on: http://{bind_url}");
    info!("Logging level: {log_level} ");
    Server::bind(&bind_url.parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await
        .expect("could not launch HTTP server on {bind_url}");
}

#[cfg(not(unix))]
async fn init_server(args: CliArgs) {
    let bind_url = &args.bind;
    let log_level = args.log_level;
    let log_path = &args.log_path;
    add_logger(log_level.clone(), log_path).expect("Unable to create logger");
    let cors = CorsLayer::new().allow_origin(Any);
    let base_router = create_base_router();
    let app = append_sysinfo_route(base_router)
        .layer(cors)
        .layer(TraceLayer::new_for_http());
    info!("Server running on: http://{bind_url}");
    info!("Logging level: {log_level} ");
    Server::bind(&bind_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("could not launch HTTP server on {bind_url}");
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use crate::server::handle_signal;
    use crate::{app::utils::cli_helper::handle_start_args, server::init_server};
    use curl::easy::Easy;
    use tracing::{debug, info};
    #[tokio::test]
    async fn test_gracefully_shutdown() {
        let args = handle_start_args();
        // TODO: finish multithread test
        let _star_platinum = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        info!("make a server");
        #[cfg(unix)]
        {
            let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
            init_server(args, shutdown_rx).await;
            debug!("wait request complete");
            handle_signal(shutdown_tx).await;
        }
        #[cfg(not(unix))]
        {
            init_server(args).await;
        }

        debug!("make a request");
        let mut curl = Easy::new();
        curl.url("http://localhost:8888/temperatures").unwrap();

        debug!("wait request complete");
        debug!("shutdown");
    }
}
