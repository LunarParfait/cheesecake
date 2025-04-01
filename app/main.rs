use std::net::SocketAddr;
use std::sync::Arc;
use environment::ENV;
use tokio::signal;
use tracing::{info, warn};
use types::app_state::AppState;
use sea_orm::{ConnectOptions, Database};
use self::logging::init_logging;

mod logging;

#[tokio::main]
async fn main() {
    ENV.init();

    init_logging().await;
    info!("Logging initialized");

    // Create database connection pool

    let mut opt = ConnectOptions::new(ENV.database_url);
    opt.max_connections(10)
        .sqlx_logging(false)
        .min_connections(10);

    let pool = Database::connect(opt).await.unwrap();

    let max_connections = pool
        .get_sqlite_connection_pool()
        .options()
        .get_max_connections();

    info!(
        "Initialized SQLite pool with {:?} max connections",
        max_connections
    );

    // If on dev environment, setup views hotwatch

    #[cfg(debug_assertions)]
    view::setup_hotwatch();

    // Create app state and router
    // TODO: continue documenting

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
    info!("All pending requests have been processed!");
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
