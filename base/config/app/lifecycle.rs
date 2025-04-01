use super::app_state::AppState;
use crate::cheesecake::lifecycle::AxumLifecycleTrait;
use anyhow::Result;
use async_trait::async_trait;
use tracing::{info, warn};

pub struct AxumLifecycle;

#[async_trait]
impl AxumLifecycleTrait for AxumLifecycle {
    async fn before_axum_start(_app_state: AppState) -> Result<()> {
        Ok(())
    }

    async fn after_axum_start(_app_state: AppState) -> Result<()> {
        Ok(())
    }

    async fn before_axum_close(_app_state: AppState) -> Result<()> {
        warn!("The server is shutting down!");
        info!("Waiting for pending requests (max. 15s)...");
        Ok(())
    }

    async fn after_axum_close(app_state: AppState) -> Result<()> {
        info!("All pending requests have been processed!");
        app_state.pool.close_by_ref().await.unwrap();
        Ok(())
    }
}
