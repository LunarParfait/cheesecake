use crate::app::app_state::AppState;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AxumLifecycleTrait {
    async fn before_axum_start(app_state: AppState) -> Result<()>;
    async fn after_axum_start(app_state: AppState) -> Result<()>;
    async fn before_axum_close(app_state: AppState) -> Result<()>;
    async fn after_axum_close(app_state: AppState) -> Result<()>;
}
