use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;

use self::migrator::Migrator;

mod migrator;
pub mod entities;
pub mod repos;

#[cfg(test)]
pub async fn init_memdb() -> Result<DatabaseConnection, DbErr> {
    init_sqlite("sqlite::memory:").await
}

pub async fn init_sqlite(conn_str: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(conn_str).await
}

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::refresh(db).await
}
