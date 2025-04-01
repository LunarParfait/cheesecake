use config::app::app_state::AppStateFactory;
use config::app::env::ENV;
use config::app::lifecycle::AxumLifecycle;
use config::velvet::app_state::AppStateFactoryTrait;
use config::velvet::env::BASE_ENV;
use config::velvet::lifecycle::AxumLifecycleTrait;
use config::velvet::logging::init_logging;
use config::velvet::routing;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    dotenvy::from_filename_override(".env.local").unwrap();

    BASE_ENV.init();
    ENV.init();

    init_logging().await;
    info!("Logging initialized");

    #[cfg(debug_assertions)]
    config::velvet::view::setup_hotwatch();

    let app_state = AppStateFactory::create().await;
    let app = routing::router(app_state.clone())
        .merge(controllers::router())
        .with_state(app_state.clone());

    let sock_addr = SocketAddr::from((BASE_ENV.hostname, BASE_ENV.port));
    let listener = TcpListener::bind(sock_addr).await.unwrap();

    AxumLifecycle::before_axum_start(app_state.clone())
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    let app_state_cloned = app_state.clone();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let app_state = app_state_cloned;

            AxumLifecycle::after_axum_start(app_state.clone())
                .await
                .unwrap();

            shutdown_signal().await;

            AxumLifecycle::before_axum_close(app_state).await.unwrap();
        })
        .await
        .unwrap();

    AxumLifecycle::after_axum_close(app_state).await.unwrap();
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
