use std::sync::Arc;
use model::app_state::AppState;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    tracing_subscriber::fmt::init();

    let db_str = dotenvy::var("DATABASE_URL").unwrap();
    let mig_dir = dotenvy::var("MIGRATIONS_DIR").unwrap();

    let pool = database::init_sqlite(&db_str).await.unwrap();
    let max_connections = pool.options().get_max_connections();
    info!(
        "Initialized SQLite pool with {:?} max connections",
        max_connections
    );

    database::migrate(&pool, &mig_dir).await.unwrap();

    #[cfg(debug_assertions)]
    view::setup_hotwatch();

    let app_state = Arc::new(AppState { pool });
    let app = controller::router(max_connections).with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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
