use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;

use self::migrator::Migrator;

mod migrator;
pub mod entities;

#[cfg(test)]
pub async fn init_memdb() -> Result<DatabaseConnection, DbErr> {
    init_sqlite("sqlite::memory:").await
}

pub async fn init_sqlite(conn_str: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(conn_str);
    opt.max_connections(10)
        .sqlx_logging(false)
        .min_connections(10);

    Database::connect(opt).await
}

pub async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
