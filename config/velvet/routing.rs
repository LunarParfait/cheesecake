use crate::app::app_state::AppState;
use axum::Router;
#[cfg(debug_assertions)]
use axum::extract::ws::WebSocket;
#[cfg(debug_assertions)]
use axum::extract::ws::WebSocketUpgrade;
#[cfg(debug_assertions)]
use axum::response::IntoResponse;
#[cfg(debug_assertions)]
use axum::routing::any;
use tower_http::services::ServeDir;

pub fn router(app_state: AppState) -> Router<AppState> {
    let serve_dir = if cfg!(debug_assertions) {
        "resources/static"
    } else {
        "dist/static"
    };

    let router = crate::app::routing::router(app_state);

    #[cfg(debug_assertions)]
    let router = router.route("/dev-server", any(dev_server));

    router.nest_service("/static", ServeDir::new(serve_dir))
}

#[cfg(debug_assertions)]
async fn dev_server(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(dev_socket)
}

#[cfg(debug_assertions)]
async fn dev_socket(mut socket: WebSocket) {
    use super::view::HOTWATCH_CHANNEL;
    use axum::extract::ws::Message;

    let mut receiver = HOTWATCH_CHANNEL.0.subscribe();
    while receiver.changed().await.is_ok() {
        if socket.send(Message::binary("")).await.is_err() {
            return;
        }
    }
}
