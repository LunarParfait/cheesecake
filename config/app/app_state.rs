use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use crate::velvet::app_state::AppStateFactoryTrait;
use crate::velvet::env::BASE_ENV;

#[derive(Debug)]
pub struct AppStateStruct {
    pub pool: DatabaseConnection,
}

pub type AppState = Arc<AppStateStruct>;
pub struct AppStateFactory;

#[async_trait]
impl AppStateFactoryTrait<AppState> for AppStateFactory {
    async fn create() -> AppState {
        let mut opt = ConnectOptions::new(BASE_ENV.database_url);
        opt.max_connections(10)
            .sqlx_logging(false)
            .min_connections(10);

        let pool = Database::connect(opt).await.unwrap();

        Arc::new(AppStateStruct { pool })
    }
}
