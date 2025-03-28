use sqlx::migrate::{MigrateError, Migrator};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::path::Path;
use thiserror::Error;

pub mod repos;

pub type DbResult<T> = Result<T, sqlx::Error>;
pub type AppTransaction<'c> = Transaction<'c, Sqlite>;

#[derive(Debug, Error)]
pub enum InitDbError {
    #[error("Database already initialized")]
    AlreadyInitialized,
    #[error("{0}")]
    SqlxError(sqlx::Error),
}

#[cfg(test)]
pub async fn init_memdb() -> Result<SqlitePool, InitDbError> {
    init_sqlite("sqlite::memory:").await
}

pub async fn init_sqlite(conn_str: &str) -> Result<SqlitePool, InitDbError> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .connect(conn_str)
        .await
        .map_err(InitDbError::SqlxError)
}

#[derive(Debug, thiserror::Error)]
pub enum AppMigrateError {
    #[error("Error running migration: {0}")]
    SqlxMigrateError(MigrateError),
    #[error("Error setting pragma: {0}")]
    PragmaError(sqlx::Error),
}

impl From<MigrateError> for AppMigrateError {
    fn from(value: MigrateError) -> Self {
        Self::SqlxMigrateError(value)
    }
}

impl From<sqlx::Error> for AppMigrateError {
    fn from(value: sqlx::Error) -> Self {
        Self::PragmaError(value)
    }
}

pub async fn migrate(
    pool: &SqlitePool,
    migrations_dir: &str,
) -> Result<(), AppMigrateError> {
    let m = Migrator::new(Path::new(migrations_dir)).await?;

    sqlx::raw_sql("PRAGMA journal_mode = WAL")
        .execute(pool)
        .await
        .unwrap();

    sqlx::raw_sql("PRAGMA busy_timeout = 5000")
        .execute(pool)
        .await
        .unwrap();

    m.run(pool).await.map_err(|e| e.into())
}
