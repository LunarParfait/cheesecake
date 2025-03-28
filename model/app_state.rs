use sqlx_sqlite::SqlitePool;

#[derive(Debug)]
pub struct AppState {
    pub pool: SqlitePool,
}
