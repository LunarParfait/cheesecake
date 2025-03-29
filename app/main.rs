use std::net::SocketAddr;
use std::sync::Arc;
use environment::ENV;
use tokio::signal;
use tracing::{debug, info, warn};
use types::app_state::AppState;

use self::logging::init_logging;

mod logging;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    ENV.init();

    init_logging().await;

    info!("Logging initialized");

    let pool = database::init_sqlite(ENV.database_url).await.unwrap();
    let max_connections = pool
        .get_sqlite_connection_pool()
        .options()
        .get_max_connections();

    info!(
        "Initialized SQLite pool with {:?} max connections",
        max_connections
    );

    database::migrate(&pool).await.unwrap();

    if args.contains(&"--migrate-only".to_string()) {
        info!("Migration cuccessful");
        return;
    }

    #[cfg(debug_assertions)]
    view::setup_hotwatch();

    let app_state = Arc::new(AppState { pool });
    let app = controller::router(max_connections).with_state(app_state.clone());

    let sock_addr = SocketAddr::from((ENV.hostname, ENV.port));
    let listener = tokio::net::TcpListener::bind(sock_addr)
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_signal().await;
            before_axum();
        })
        .await
        .unwrap();
    after_axum(app_state).await;
}

fn before_axum() {
    warn!("The server is shutting down!");
    info!("Waiting for pending requests (max. 15s)...");
}

async fn after_axum(app_state: Arc<AppState>) {
    debug!("All pending requests have been processed!");
    app_state.pool.close_by_ref().await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
